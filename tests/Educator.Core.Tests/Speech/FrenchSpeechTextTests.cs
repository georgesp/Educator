using Educator.Core.Speech;

namespace Educator.Core.Tests.Speech;

[Trait("Category", "Unit")]
public sealed class FrenchSpeechTextTests
{
    [Theory]
    [InlineData(0, "zéro")]
    [InlineData(1, "un")]
    [InlineData(16, "seize")]
    [InlineData(17, "dix-sept")]
    [InlineData(21, "vingt et un")]
    [InlineData(22, "vingt-deux")]
    [InlineData(30, "trente")]
    [InlineData(61, "soixante et un")]
    [InlineData(70, "soixante-dix")]
    [InlineData(71, "soixante et onze")]
    [InlineData(77, "soixante-dix-sept")]
    [InlineData(80, "quatre-vingts")]
    [InlineData(81, "quatre-vingt-un")]
    [InlineData(90, "quatre-vingt-dix")]
    [InlineData(91, "quatre-vingt-onze")]
    [InlineData(99, "quatre-vingt-dix-neuf")]
    [InlineData(100, "cent")]
    [InlineData(101, "cent un")]
    [InlineData(180, "cent quatre-vingts")]
    [InlineData(200, "deux cents")]
    [InlineData(201, "deux cent un")]
    [InlineData(371, "trois cent soixante et onze")]
    [InlineData(999, "neuf cent quatre-vingt-dix-neuf")]
    [InlineData(1000, "mille")]
    [InlineData(1001, "mille un")]
    [InlineData(2000, "deux mille")]
    [InlineData(80000, "quatre-vingt mille")]
    [InlineData(200300, "deux cent mille trois cents")]
    [InlineData(1_000_000, "un million")]
    public void NumberToWords(int number, string words)
        => Assert.Equal(words, FrenchSpeechText.NumberToWords(number));

    [Theory]
    [InlineData("7 + 5", "sept plus cinq")]
    [InlineData("7 + 5 = 12", "sept plus cinq égale douze")]
    [InlineData("12 − 5 = 7", "douze moins cinq égale sept")]
    [InlineData("12 - 5", "douze moins cinq")]
    [InlineData("7 + ? = 12", "sept plus combien égale douze")]
    [InlineData("? − 5 = 7", "combien moins cinq égale sept")]
    [InlineData("Combien font 7 plus 5 ?", "Combien font sept plus cinq ?")]
    [InlineData("Bravo ! Tu as 9 bonnes réponses sur 10.", "Bravo ! Tu as neuf bonnes réponses sur dix.")]
    [InlineData("Peut-être 245 + 437", "Peut-être deux cent quarante-cinq plus quatre cent trente-sept")]
    public void Normalize(string text, string spoken)
        => Assert.Equal(spoken, FrenchSpeechText.Normalize(text));
}
