package questions;

public interface Question<T> {
    void ask();
    void showAnswer();
    boolean checkAnswer(T answer);
    Type getQuestionType();
}
