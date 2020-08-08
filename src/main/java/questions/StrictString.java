package questions;

public class StrictString implements Question<String> {
    private Type questionType;
    private String questionText;
    private String answer;

    public StrictString(Type type, String text, String answer) {
        this.setQuestionType(type);
        this.setQuestionText(text);
        this.setAnswer(answer);
    }

    public void ask() {
        System.out.println(this.questionText);
    }

    public void showAnswer() {
        System.out.println(this.getAnswer());
    }

    public boolean checkAnswer(String usersAnswer) {
        return (usersAnswer.equals(this.getAnswer()));
    }

    public Type getQuestionType() {
        return questionType;
    }

    private void setQuestionType(Type questionType) {
        this.questionType = questionType;
    }

    public String getQuestionText() {
        return questionText;
    }

    private void setQuestionText(String questionText) {
        this.questionText = questionText;
    }

    public String getAnswer() {
        return answer;
    }

    private void setAnswer(String answer) {
        this.answer = answer;
    }
}
