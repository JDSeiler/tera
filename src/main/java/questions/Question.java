package questions;

public interface Question<T> {
    void ask();
    void showAnswer();
    boolean checkAnswer(T answer);
    String getQuestionText();
    Type getQuestionType();
}
