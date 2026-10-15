// System voice of the iPad for Educator (SUB023805, ADR 0005): the French voices of the device and
// the synthesis of a text to 16-bit PCM samples, turned into a WAV by the Rust side. No business rule:
// texts, speeds and cutting are decided by the core.

import AVFoundation
import Tauri
import UIKit
import WebKit

/// Arguments of `synthesize`.
class SynthesizeArgs: Decodable {
  /// Plain text, read when SSML is not available.
  let text: String
  /// The same text in SSML (pauses after colons, ADR 0005).
  let ssml: String?
  /// 1 = normal speed, from 0.5 to 1.5 (as on the desktop).
  let speed: Double
  /// Identifier of a voice (`AVSpeechSynthesisVoice.identifier`); the best French voice when absent.
  let voice: String?
}

struct VoiceInfo: Encodable {
  let id: String
  let name: String
  let language: String
  /// "default", "enhanced" or "premium".
  let quality: String
}

struct VoicesResponse: Encodable {
  let voices: [VoiceInfo]
  /// Voice used when none is asked for: the best French (France) voice, or none.
  let chosen: VoiceInfo?
}

struct SynthesizeResponse: Encodable {
  /// 16-bit little-endian mono samples, in base64.
  let samples: String
  let sampleRate: Int
  let voice: String
  /// Time taken by the synthesis.
  let milliseconds: Int
}

/// Longest wait for a synthesis before it is given up.
private let timeoutSeconds = 15.0

/// One synthesis in progress: keeps its synthesizer alive until the last buffer.
final class WriteJob: NSObject, AVSpeechSynthesizerDelegate {
  let synthesizer = AVSpeechSynthesizer()
  private let invoke: Invoke
  private let voiceName: String
  private let started = Date()
  private let lock = NSLock()
  private var samples = Data()
  private var sampleRate: Double = 0
  private var finished = false
  var onDone: ((WriteJob) -> Void)?

  init(invoke: Invoke, voiceName: String) {
    self.invoke = invoke
    self.voiceName = voiceName
    super.init()
    synthesizer.delegate = self
  }

  func start(_ utterance: AVSpeechUtterance) {
    synthesizer.write(utterance) { [weak self] buffer in
      guard let self = self, let pcm = buffer as? AVAudioPCMBuffer else { return }
      if pcm.frameLength == 0 {
        // An empty buffer marks the end of the reading.
        self.finish(error: nil)
      } else {
        self.append(pcm)
      }
    }
    DispatchQueue.global().asyncAfter(deadline: .now() + timeoutSeconds) { [weak self] in
      self?.finish(error: "The system voice did not answer within \(Int(timeoutSeconds)) s.")
    }
  }

  private func append(_ pcm: AVAudioPCMBuffer) {
    let frames = Int(pcm.frameLength)
    lock.lock()
    defer { lock.unlock() }
    if finished {
      return
    }
    sampleRate = pcm.format.sampleRate
    if let int16 = pcm.int16ChannelData {
      samples.append(UnsafeBufferPointer(start: int16[0], count: frames))
    } else if let float = pcm.floatChannelData {
      var converted = [Int16](repeating: 0, count: frames)
      for index in 0..<frames {
        let value = float[0][index]
        converted[index] = value.isNaN ? 0 : Int16(max(-1, min(1, value)) * 32767)
      }
      converted.withUnsafeBufferPointer { samples.append($0) }
    } else if let int32 = pcm.int32ChannelData {
      var converted = [Int16](repeating: 0, count: frames)
      for index in 0..<frames {
        converted[index] = Int16(truncatingIfNeeded: int32[0][index] >> 16)
      }
      converted.withUnsafeBufferPointer { samples.append($0) }
    }
  }

  func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
    // Fallback when no empty buffer comes: leave time for the last buffers.
    DispatchQueue.global().asyncAfter(deadline: .now() + 0.5) { [weak self] in
      self?.finish(error: nil)
    }
  }

  func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
    finish(error: "The system voice stopped the reading.")
  }

  func finish(error: String?) {
    lock.lock()
    if finished {
      lock.unlock()
      return
    }
    finished = true
    let data = samples
    let rate = sampleRate
    lock.unlock()

    if let error = error {
      invoke.reject(error)
    } else if data.isEmpty || rate <= 0 {
      invoke.reject("The system voice produced no sound.")
    } else {
      invoke.resolve(
        SynthesizeResponse(
          samples: data.base64EncodedString(),
          sampleRate: Int(rate),
          voice: voiceName,
          milliseconds: Int(Date().timeIntervalSince(started) * 1000)))
    }
    onDone?(self)
  }
}

class SystemVoicePlugin: Plugin {
  private let jobsLock = NSLock()
  private var jobs: [ObjectIdentifier: WriteJob] = [:]

  public override func load(webview: WKWebView) {
    activateAudioSession()
    // After a call or another app that took the sound, and when the app comes back to the
    // foreground: the session is set again, so that the voice is heard at once (SUB023806).
    NotificationCenter.default.addObserver(
      self, selector: #selector(audioInterrupted(_:)),
      name: AVAudioSession.interruptionNotification, object: nil)
    NotificationCenter.default.addObserver(
      self, selector: #selector(becameActive),
      name: UIApplication.didBecomeActiveNotification, object: nil)
  }

  /// Instructions and dictations are heard even when the iPad is in silent mode (ADR 0005), like
  /// the sound of a video. The WAV played by the WebView follows this session.
  private func activateAudioSession() {
    do {
      try AVAudioSession.sharedInstance().setCategory(.playback, mode: .spokenAudio, options: [])
      try AVAudioSession.sharedInstance().setActive(true)
    } catch {
      NSLog("Educator system voice: audio session not set: \(error)")
    }
  }

  @objc private func audioInterrupted(_ notification: Notification) {
    guard let raw = notification.userInfo?[AVAudioSessionInterruptionTypeKey] as? UInt,
      let type = AVAudioSession.InterruptionType(rawValue: raw), type == .ended
    else {
      return
    }
    activateAudioSession()
  }

  @objc private func becameActive() {
    activateAudioSession()
  }

  @objc public func voices(_ invoke: Invoke) throws {
    let french = AVSpeechSynthesisVoice.speechVoices().filter { $0.language.hasPrefix("fr") }
    invoke.resolve(
      VoicesResponse(
        voices: french.map(SystemVoicePlugin.info),
        chosen: SystemVoicePlugin.bestFrenchVoice().map(SystemVoicePlugin.info)))
  }

  @objc public func synthesize(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(SynthesizeArgs.self)
    let asked = args.voice.flatMap { AVSpeechSynthesisVoice(identifier: $0) }
    guard let voice = asked ?? SystemVoicePlugin.bestFrenchVoice() else {
      invoke.reject("No French (France) voice is installed on the iPad.")
      return
    }

    var fromSsml: AVSpeechUtterance?
    if let ssml = args.ssml {
      if #available(iOS 16.0, macOS 13.0, *) {
        fromSsml = AVSpeechUtterance(ssmlRepresentation: ssml)
      }
    }
    let utterance = fromSsml ?? AVSpeechUtterance(string: args.text)
    utterance.voice = voice
    utterance.rate = SystemVoicePlugin.rate(for: args.speed)

    let job = WriteJob(invoke: invoke, voiceName: voice.name)
    let key = ObjectIdentifier(job)
    jobsLock.lock()
    jobs[key] = job
    jobsLock.unlock()
    job.onDone = { [weak self] _ in
      guard let self = self else { return }
      self.jobsLock.lock()
      self.jobs.removeValue(forKey: key)
      self.jobsLock.unlock()
    }
    job.start(utterance)
  }

  /// 1 = the default rate of the system; 0.5 = twice slower; kept within the limits of the system.
  static func rate(for speed: Double) -> Float {
    let rate = AVSpeechUtteranceDefaultSpeechRate * Float(speed)
    return max(AVSpeechUtteranceMinimumSpeechRate, min(AVSpeechUtteranceMaximumSpeechRate, rate))
  }

  /// The best voice of France (premium, then enhanced, then default); Canadian voices are left out.
  static func bestFrenchVoice() -> AVSpeechSynthesisVoice? {
    AVSpeechSynthesisVoice.speechVoices()
      .filter { $0.language == "fr-FR" }
      .max { $0.quality.rawValue < $1.quality.rawValue }
  }

  static func info(_ voice: AVSpeechSynthesisVoice) -> VoiceInfo {
    let quality: String
    switch voice.quality.rawValue {
    case 3: quality = "premium"
    case 2: quality = "enhanced"
    default: quality = "default"
    }
    return VoiceInfo(id: voice.identifier, name: voice.name, language: voice.language, quality: quality)
  }
}

@_cdecl("init_plugin_system_voice")
func initPlugin() -> Plugin {
  return SystemVoicePlugin()
}
