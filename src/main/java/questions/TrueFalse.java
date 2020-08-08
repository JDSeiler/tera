package questions;

public class TrueFalse implements Question<Boolean> {
    private Type questionType;
    private String questionText;
    private Boolean answer;

    public TrueFalse(Type type, String text, Boolean answer) {
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

    public boolean checkAnswer(Boolean usersAnswer) {
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

    public Boolean getAnswer() {
        return answer;
    }

    private void setAnswer(Boolean answer) {
        this.answer = answer;
    }
}
