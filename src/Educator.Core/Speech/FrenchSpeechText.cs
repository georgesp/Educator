using System.Globalization;
using System.Text;
using System.Text.RegularExpressions;

namespace Educator.Core.Speech;

/// <summary>
/// Prepares a text for the speech engine (SUB000111): numbers become French words and the calculation
/// symbols become words (<c>+</c> « plus », <c>−</c> « moins », <c>=</c> « égale », a <c>?</c> standing for a
/// number « combien »), so that the voice reads « sept plus cinq égale douze ».
/// </summary>
public static partial class FrenchSpeechText
{
    private static readonly string[] Units =
    [
        "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
        "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf",
    ];

    private static readonly string[] Tens = ["", "", "vingt", "trente", "quarante", "cinquante", "soixante"];

    public static string Normalize(string text)
    {
        ArgumentNullException.ThrowIfNull(text);

        var result = MissingNumber().Replace(text, " combien ");
        result = result
            .Replace("+", " plus ", StringComparison.Ordinal)
            .Replace("−", " moins ", StringComparison.Ordinal)
            .Replace("–", " moins ", StringComparison.Ordinal)
            .Replace("=", " égale ", StringComparison.Ordinal)
            .Replace("×", " fois ", StringComparison.Ordinal);
        result = Subtraction().Replace(result, " moins ");
        result = Number().Replace(result, m => NumberToWords(int.Parse(m.Value, NumberStyles.None, CultureInfo.InvariantCulture)));
        result = Spaces().Replace(result, " ").Trim();
        return SpaceBeforePunctuation().Replace(result, "$1");
    }

    /// <summary>French words of a number from 0 to 999 999 999 (« soixante et onze », « quatre-vingts »).</summary>
    public static string NumberToWords(int number)
    {
        ArgumentOutOfRangeException.ThrowIfNegative(number);
        ArgumentOutOfRangeException.ThrowIfGreaterThan(number, 999_999_999);

        if (number == 0)
        {
            return Units[0];
        }

        var parts = new List<string>();
        var millions = number / 1_000_000;
        var thousands = number / 1000 % 1000;
        var rest = number % 1000;
        if (millions > 0)
        {
            parts.Add(BelowThousand(millions, isFinal: true) + (millions > 1 ? " millions" : " million"));
        }

        if (thousands > 0)
        {
            // « mille » never takes an s, and « cent » / « vingt » stay singular before it.
            parts.Add(thousands == 1 ? "mille" : BelowThousand(thousands, isFinal: false) + " mille");
        }

        if (rest > 0)
        {
            parts.Add(BelowThousand(rest, isFinal: true));
        }

        return string.Join(' ', parts);
    }

    private static string BelowThousand(int number, bool isFinal)
    {
        var hundreds = number / 100;
        var rest = number % 100;
        var builder = new StringBuilder();
        if (hundreds > 0)
        {
            builder.Append(hundreds == 1 ? "cent" : Units[hundreds] + " cent");
            if (hundreds > 1 && rest == 0 && isFinal)
            {
                builder.Append('s');
            }
        }

        if (rest > 0)
        {
            if (builder.Length > 0)
            {
                builder.Append(' ');
            }

            builder.Append(BelowHundred(rest, isFinal));
        }

        return builder.ToString();
    }

    private static string BelowHundred(int number, bool isFinal)
    {
        if (number < 20)
        {
            return Units[number];
        }

        var ten = number / 10;
        var unit = number % 10;
        return ten switch
        {
            7 => unit == 1 ? "soixante et onze" : "soixante-" + Units[10 + unit],
            8 => unit == 0 ? (isFinal ? "quatre-vingts" : "quatre-vingt") : "quatre-vingt-" + Units[unit],
            9 => "quatre-vingt-" + Units[10 + unit],
            _ => unit switch
            {
                0 => Tens[ten],
                1 => Tens[ten] + " et un",
                _ => Tens[ten] + "-" + Units[unit],
            },
        };
    }

    /// <summary>A <c>?</c> next to a calculation sign stands for a number: <c>7 + ? = 12</c>.</summary>
    [GeneratedRegex(@"(?<=[+\-−–=×]\s*)\?|\?(?=\s*[+\-−–=×])")]
    private static partial Regex MissingNumber();

    /// <summary>A hyphen between two numbers is a minus sign; hyphens inside words are kept.</summary>
    [GeneratedRegex(@"(?<=\d\s*)-(?=\s*\d)")]
    private static partial Regex Subtraction();

    [GeneratedRegex(@"\d{1,9}")]
    private static partial Regex Number();

    [GeneratedRegex(@"\s+")]
    private static partial Regex Spaces();

    [GeneratedRegex(@"\s+([,.])")]
    private static partial Regex SpaceBeforePunctuation();
}
