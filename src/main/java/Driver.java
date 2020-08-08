import answers.Answer;
import questions.Question;
import questions.Type;
import util.AnswerFetcher;
import util.JsonLoader;

import javax.json.JsonArray;
import javax.json.JsonObject;
import java.util.ArrayList;

public class Driver {
    public static void main(String[] args) {
        JsonObject test = JsonLoader.loadJsonFromQuestionSet(args[0]);
        JsonArray jsonQuestions = test.getJsonArray("questions");
        AnswerFetcher fetcher = new AnswerFetcher();

        ArrayList<Question> questionList = JsonLoader.createQuestionList(jsonQuestions);

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
                System.out.println();
            }
        }
    }
}
