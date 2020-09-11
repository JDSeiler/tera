import answers.Answer;
import questions.Question;
import util.AnswerFetcher;
import util.JsonLoader;

import javax.json.JsonArray;
import javax.json.JsonObject;
import java.util.ArrayList;

public class Driver {
    /*
    * Improvement plan:
    * 1. Remove the type enum in favor of instanceof
    * 2. Remove the generic parameter in Question by:
    *   a. Changing the parameter of checkAnswer to type String
    *   b. Move all the type-checking logic into each concrete subclass of Question
    * */
    public static void main(String[] args) {
        JsonObject test = JsonLoader.loadJsonFromQuestionSet(args[0]);
        JsonArray jsonQuestions = test.getJsonArray("questions");
        AnswerFetcher fetcher = new AnswerFetcher();

        ArrayList<Question> questionList = JsonLoader.createQuestionList(jsonQuestions);

        int incorrectQuestions = 0;
        ArrayList<String> missedQuestions = new ArrayList<>();
        for (int i = 0; i < questionList.size(); i++) {
            Question current = questionList.get(i);
            System.out.printf("Question %d of %d:%n", i+1, questionList.size());

            current.ask();
            Answer userAnswer = fetcher.fetchAnswer(current.getQuestionType());
            boolean answerIsCorrect = current.checkAnswer(userAnswer.getValue());
            if (answerIsCorrect) {
                System.out.printf("Correct%n%n");
            } else {
                System.out.printf("Incorrect, the correct answer was:%n");
                current.showAnswer();
                incorrectQuestions++;
                missedQuestions.add(current.getQuestionText());
                System.out.println();
            }
        }

        System.out.println("+============+");
        System.out.println("|   Report   |");
        System.out.println("+============+");

        System.out.printf("You missed %d questions out of %d%n", incorrectQuestions, questionList.size());
        System.out.printf("Grade: %d%n", Math.round((1 - (float) incorrectQuestions / (float) questionList.size()) * 100));
        System.out.println("Here are the questions you missed: ");
        for (String question : missedQuestions) {
            System.out.printf("- %s%n", question);
        }
    }
}
