using System.Globalization;

namespace Educator.Core.Exercises;

public enum ArithmeticOperation
{
    Addition,
    Subtraction,
}

/// <summary>A calculation <c>Left op Right = Result</c>, with natural numbers only.</summary>
public readonly record struct Calculation(ArithmeticOperation Operation, int Left, int Right)
{
    public int Result => Operation == ArithmeticOperation.Addition ? Left + Right : Left - Right;

    /// <summary>Symbol shown on screen (the true minus sign for subtractions).</summary>
    public string Symbol => Operation == ArithmeticOperation.Addition ? "+" : "−";

    /// <summary>
    /// Key used to avoid duplicates in a series: an addition and its commuted form are the same exercise
    /// (7 + 5 and 5 + 7), whatever the format.
    /// </summary>
    public (ArithmeticOperation Operation, int A, int B) DuplicateKey => Operation == ArithmeticOperation.Addition
        ? (Operation, Math.Min(Left, Right), Math.Max(Left, Right))
        : (Operation, Left, Right);

    /// <summary>Carry (addition) or borrow (subtraction) in at least one column.</summary>
    public bool HasCarry
    {
        get
        {
            var left = Left;
            var right = Right;
            var carry = 0;
            while (left > 0 || right > 0)
            {
                var top = (left % 10) - (Operation == ArithmeticOperation.Subtraction ? carry : 0);
                var bottom = right % 10;
                if (Operation == ArithmeticOperation.Addition)
                {
                    carry = top + bottom + carry >= 10 ? 1 : 0;
                }
                else
                {
                    carry = top < bottom ? 1 : 0;
                }

                if (carry == 1)
                {
                    return true;
                }

                left /= 10;
                right /= 10;
            }

            return false;
        }
    }

    /// <summary>Complete equality, e.g. <c>7 + 5 = 12</c>.</summary>
    public override string ToString()
        => string.Create(CultureInfo.InvariantCulture, $"{Left} {Symbol} {Right} = {Result}");
}
