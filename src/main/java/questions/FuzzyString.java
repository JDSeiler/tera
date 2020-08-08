package questions;

import me.xdrop.fuzzywuzzy.FuzzySearch;

public class FuzzyString implements Question<String> {
    private Type questionType;
    private String questionText;
    private String answer;

    public FuzzyString(Type type, String text, String answer) {
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
        int score = FuzzySearch.weightedRatio(usersAnswer, this.getAnswer());
        // Matches are graded from 0 -> 100 with 100 being an exact match
        // for the given fuzzy algorithm. 95 is just an arbitrary tolerance that feels right.
        // I'd like to make it configurable but for now it's just a magic number.
        return (score > 95);
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
