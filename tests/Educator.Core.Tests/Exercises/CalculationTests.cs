using Educator.Core.Exercises;

namespace Educator.Core.Tests.Exercises;

[Trait("Category", "Unit")]
public sealed class CalculationTests
{
    [Theory]
    [InlineData(3, 4, false)]
    [InlineData(7, 5, true)]
    [InlineData(23, 14, false)]
    [InlineData(28, 14, true)]
    [InlineData(150, 60, true)]
    [InlineData(245, 437, true)]
    [InlineData(40, 9, false)]
    public void Addition_Carry(int left, int right, bool carry)
        => Assert.Equal(carry, new Calculation(ArithmeticOperation.Addition, left, right).HasCarry);

    [Theory]
    [InlineData(9, 4, false)]
    [InlineData(12, 5, true)]
    [InlineData(57, 23, false)]
    [InlineData(50, 6, true)]
    [InlineData(523, 7, true)]
    [InlineData(157, 5, false)]
    [InlineData(746, 80, true)]
    [InlineData(100, 1, true)]
    public void Subtraction_Borrow(int left, int right, bool borrow)
        => Assert.Equal(borrow, new Calculation(ArithmeticOperation.Subtraction, left, right).HasCarry);

    [Fact]
    public void DuplicateKey_CommutedAdditionsAreTheSame()
    {
        Assert.Equal(
            new Calculation(ArithmeticOperation.Addition, 7, 5).DuplicateKey,
            new Calculation(ArithmeticOperation.Addition, 5, 7).DuplicateKey);
        Assert.NotEqual(
            new Calculation(ArithmeticOperation.Subtraction, 7, 5).DuplicateKey,
            new Calculation(ArithmeticOperation.Addition, 7, 5).DuplicateKey);
    }

    [Fact]
    public void ToString_IsTheCompleteEquality()
    {
        Assert.Equal("7 + 5 = 12", new Calculation(ArithmeticOperation.Addition, 7, 5).ToString());
        Assert.Equal("12 − 5 = 7", new Calculation(ArithmeticOperation.Subtraction, 12, 5).ToString());
    }

    [Fact]
    public void Exercise_HidesTheAskedNumber()
    {
        var calculation = new Calculation(ArithmeticOperation.Addition, 7, 5);

        var result = new Exercise(0, calculation, HiddenPart.Result, null);
        var missingRight = new Exercise(1, calculation, HiddenPart.Right, null);
        var missingLeft = new Exercise(2, calculation, HiddenPart.Left, null);

        Assert.Equal("7 + 5 = ?", result.Prompt);
        Assert.Equal(12, result.Expected);
        Assert.Equal("7 + ? = 12", missingRight.Prompt);
        Assert.Equal(5, missingRight.Expected);
        Assert.Equal("? + 5 = 12", missingLeft.Prompt);
        Assert.Equal(7, missingLeft.Expected);
        Assert.Equal("7 + 5 = 12", missingLeft.Explanation);
        Assert.Equal("Combien font 7 plus 5 ?", result.SpokenPrompt);
        Assert.Equal("Combien faut-il ajouter à 7 pour obtenir 12 ?", missingRight.SpokenPrompt);
    }
}
