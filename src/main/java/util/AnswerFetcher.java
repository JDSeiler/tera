package util;

import answers.Answer;
import questions.Type;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Scanner;

public class AnswerFetcher {
    private final Scanner console;
    private final String prompt = "> ";
    private final Collection<String> trueAnswers = new ArrayList<>();
    private final Collection<String> falseAnswers = new ArrayList<>();

    public AnswerFetcher() {
        this.console = new Scanner(System.in);

        trueAnswers.add("1");
        trueAnswers.add("y");
        trueAnswers.add("yes");
        trueAnswers.add("t");
        trueAnswers.add("true");

        falseAnswers.add("0");
        falseAnswers.add("n");
        falseAnswers.add("no");
        falseAnswers.add("f");
        falseAnswers.add("false");
    }

    private void printPrompt() {
        System.out.print(this.prompt);
    }

    private String getRawLine() {
        this.printPrompt();
        return this.console.nextLine();
    }

    public Answer<?> fetchAnswer(Type questionType) {
        switch (questionType) {
            case FuzzyString:
            case StrictString:
                return this.fetchStringAnswer();
            case MultipleChoice:
                return this.fetchMultipleChoiceAnswer();
            case TrueFalse:
                return this.fetchBooleanAnswer();
            case Invalid:
            default:
                return null;
        }
    }

    public Answer<String> fetchStringAnswer() {
        return new Answer<>(this.getRawLine().trim());
    }

    public Answer<Integer> fetchMultipleChoiceAnswer() {
        String trimmedAnswer = this.getRawLine().trim();
        int humanIndex = Integer.parseInt(trimmedAnswer);
        // Answer should be an index into the choices array, so subtract 1
        return new Answer<>(humanIndex - 1);
    }

    public Answer<Boolean> fetchBooleanAnswer() {
        String cleanedAnswer = this.getRawLine().trim().toLowerCase();
        if (this.trueAnswers.contains(cleanedAnswer)) {
            return new Answer<>(true);
        } else if (this.falseAnswers.contains(cleanedAnswer)) {
            return new Answer<>(false);
        } else {
            // Totally invalid answers will get their own exception
            // type later that I will use in the main loop, but that's
            // a feature for later.
            return new Answer<>(null);
        }
    }
}
