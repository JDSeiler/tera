package questions;

import java.util.ArrayList;

public class MultipleChoice implements Question<Integer> {
    private Type questionType;
    private String questionText;
    private ArrayList<String> choices;
    private Integer answer;

    public MultipleChoice(Type type, String text, ArrayList<String> choices, Integer answer) {
        this.setQuestionType(type);
        this.setQuestionText(text);
        this.setChoices(choices);
        this.setAnswer(answer);

        if (this.getAnswer() > this.getChoices().size()-1) {
            System.out.println("Invalid index in multiple choice question!");
            System.out.printf("Index %d outside the range of %d%n", this.getAnswer(), this.getChoices().size()-1);
            System.out.printf("Question text: %s%n", this.getQuestionText());
        }
    }

    public void ask() {
        System.out.println(this.questionText);
        for (int i = 0; i < this.getChoices().size(); i++) {
            System.out.printf("%d. %s%n", i+1, this.getChoices().get(i));
        }
    }

    public void showAnswer() {
        // Add 1 since the UI uses a 1 based index for the choices
        System.out.printf("Option %d%n", this.getAnswer()+1);
    }

    public boolean checkAnswer(Integer usersAnswer) {
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

    public ArrayList<String> getChoices() {
        return choices;
    }

    private void setChoices(ArrayList<String> choices) {
        this.choices = choices;
    }

    public Integer getAnswer() {
        return answer;
    }

    private void setAnswer(Integer answer) {
        this.answer = answer;
    }
}
