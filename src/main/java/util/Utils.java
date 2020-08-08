package util;

import questions.Type;

public class Utils {
    public static Type getTypeFromKey(String key) {
        switch (key) {
            case "fuzzy-string":
                return Type.FuzzyString;
            case "strict-string":
                return Type.StrictString;
            case "multiple-choice":
                return Type.MultipleChoice;
            case "true-false":
                return Type.TrueFalse;
            default:
                return Type.Invalid;
        }
    }
}
