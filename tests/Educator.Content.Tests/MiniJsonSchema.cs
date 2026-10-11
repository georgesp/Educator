using System.Globalization;
using System.Text.Json.Nodes;
using System.Text.RegularExpressions;

namespace Educator.Content.Tests;

/// <summary>
/// Minimal JSON Schema (2020-12) validator for the keywords used by <c>content/schemas/*.json</c>:
/// type, required, properties, additionalProperties, items, enum, const, minimum, maximum, minLength,
/// maxLength, pattern, minItems, maxItems, uniqueItems and local <c>$ref</c>. Unknown keywords make
/// the test fail, so that a schema never silently uses something this validator ignores.
/// </summary>
internal static class MiniJsonSchema
{
    private static readonly HashSet<string> Supported =
    [
        "$schema", "$id", "$defs", "$ref", "title", "description", "default", "type", "required", "properties",
        "additionalProperties", "items", "enum", "const", "minimum", "maximum", "minLength", "maxLength", "pattern",
        "minItems", "maxItems", "uniqueItems",
    ];

    public static IReadOnlyList<string> Validate(JsonNode? instance, JsonObject schema)
    {
        var errors = new List<string>();
        Validate(instance, schema, schema, "$", errors);
        return errors;
    }

    private static void Validate(JsonNode? instance, JsonObject schema, JsonObject root, string path, List<string> errors)
    {
        foreach (var keyword in schema.Select(p => p.Key).Where(k => !Supported.Contains(k)))
        {
            errors.Add($"{path}: schema keyword '{keyword}' not supported by the test validator.");
        }

        if (schema["$ref"] is JsonValue reference)
        {
            var target = root;
            foreach (var segment in reference.GetValue<string>().TrimStart('#', '/').Split('/'))
            {
                target = (JsonObject)target[segment]!;
            }

            Validate(instance, target, root, path, errors);
        }

        if (schema["type"] is JsonValue type && !HasType(instance, type.GetValue<string>()))
        {
            errors.Add($"{path}: expected {type}.");
            return;
        }

        if (schema["enum"] is JsonArray allowed && !allowed.Any(a => JsonNode.DeepEquals(a, instance)))
        {
            errors.Add($"{path}: {instance?.ToJsonString()} is not one of {allowed.ToJsonString()}.");
        }

        if (schema["const"] is JsonNode constant && !JsonNode.DeepEquals(constant, instance))
        {
            errors.Add($"{path}: must be {constant.ToJsonString()}.");
        }

        switch (instance)
        {
            case JsonObject obj:
                ValidateObject(obj, schema, root, path, errors);
                break;
            case JsonArray array:
                ValidateArray(array, schema, root, path, errors);
                break;
            case JsonValue value when value.TryGetValue<string>(out var text):
                if (schema["minLength"] is JsonValue minLength && text.Length < minLength.GetValue<int>())
                {
                    errors.Add($"{path}: shorter than {minLength}.");
                }

                if (schema["maxLength"] is JsonValue maxLength && text.Length > maxLength.GetValue<int>())
                {
                    errors.Add($"{path}: longer than {maxLength}.");
                }

                if (schema["pattern"] is JsonValue pattern && !Regex.IsMatch(text, pattern.GetValue<string>(), RegexOptions.None, TimeSpan.FromSeconds(1)))
                {
                    errors.Add($"{path}: '{text}' does not match {pattern}.");
                }

                break;
            case JsonValue value when value.TryGetValue<double>(out var number):
                if (schema["minimum"] is JsonValue minimum && number < minimum.GetValue<double>())
                {
                    errors.Add($"{path}: {number.ToString(CultureInfo.InvariantCulture)} < {minimum}.");
                }

                if (schema["maximum"] is JsonValue maximum && number > maximum.GetValue<double>())
                {
                    errors.Add($"{path}: {number.ToString(CultureInfo.InvariantCulture)} > {maximum}.");
                }

                break;
        }
    }

    private static void ValidateObject(JsonObject obj, JsonObject schema, JsonObject root, string path, List<string> errors)
    {
        foreach (var required in schema["required"]?.AsArray() ?? [])
        {
            if (!obj.ContainsKey(required!.GetValue<string>()))
            {
                errors.Add($"{path}: missing property '{required}'.");
            }
        }

        var properties = schema["properties"]?.AsObject();
        foreach (var (name, value) in obj)
        {
            if (properties?[name] is JsonObject propertySchema)
            {
                Validate(value, propertySchema, root, $"{path}.{name}", errors);
            }
            else if (schema["additionalProperties"] is JsonValue additional && !additional.GetValue<bool>())
            {
                errors.Add($"{path}: unexpected property '{name}'.");
            }
            else if (schema["additionalProperties"] is JsonObject additionalSchema)
            {
                Validate(value, additionalSchema, root, $"{path}.{name}", errors);
            }
        }
    }

    private static void ValidateArray(JsonArray array, JsonObject schema, JsonObject root, string path, List<string> errors)
    {
        if (schema["minItems"] is JsonValue minItems && array.Count < minItems.GetValue<int>())
        {
            errors.Add($"{path}: fewer than {minItems} items.");
        }

        if (schema["maxItems"] is JsonValue maxItems && array.Count > maxItems.GetValue<int>())
        {
            errors.Add($"{path}: more than {maxItems} items.");
        }

        if (schema["uniqueItems"] is JsonValue unique && unique.GetValue<bool>()
            && array.Select(i => i?.ToJsonString()).Distinct().Count() != array.Count)
        {
            errors.Add($"{path}: items must be unique.");
        }

        if (schema["items"] is JsonObject itemSchema)
        {
            for (var i = 0; i < array.Count; i++)
            {
                Validate(array[i], itemSchema, root, $"{path}[{i}]", errors);
            }
        }
    }

    private static bool HasType(JsonNode? node, string type) => type switch
    {
        "object" => node is JsonObject,
        "array" => node is JsonArray,
        "string" => node is JsonValue v && v.TryGetValue<string>(out _),
        "integer" => node is JsonValue v && v.TryGetValue<double>(out var d) && Math.Abs(d % 1) < double.Epsilon,
        "number" => node is JsonValue v && v.TryGetValue<double>(out _),
        "boolean" => node is JsonValue v && v.TryGetValue<bool>(out _),
        "null" => node is null,
        _ => false,
    };
}
