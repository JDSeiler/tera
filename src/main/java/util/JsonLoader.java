package util;

import questions.*;

import javax.json.Json;
import javax.json.JsonArray;
import javax.json.JsonObject;
import javax.json.JsonReader;
import java.io.*;
import java.util.ArrayList;

public class JsonLoader {
    private static final String SOURCE_DIR =  System.getProperty("user.home") + "/.tera/sets";

    public static JsonObject loadJsonFromQuestionSet(String setName) {
        try {
            String pathToSet = String.format("%s/%s.json", SOURCE_DIR, setName);
            File questionSetFileHandle = new File(pathToSet);
            BufferedReader questionSetReader = new BufferedReader(new FileReader(questionSetFileHandle));
            JsonReader jsonReader = Json.createReader(questionSetReader);
            return jsonReader.readObject();
        } catch (FileNotFoundException e) {
            System.out.printf("Question set: %s not found!%n", setName);
            System.out.println(e.toString());
            return null;
        }
    }

    public static ArrayList<Question> createQuestionList(JsonArray jsonQuestionList) {
        ArrayList<Question> questionList = new ArrayList<>();
        for (int i = 0; i < jsonQuestionList.size(); i++) {
            JsonObject question = jsonQuestionList.getJsonObject(i);
            Type type = Utils.getTypeFromKey(question.getString("type"));

            switch (type) {
                case FuzzyString:
                    Question<String> fuzzyStringQuestion = new FuzzyString(type,
                            question.getString("question"),
                            question.getString("answer"));
                    questionList.add(fuzzyStringQuestion);
                    break;
                case StrictString:
                    Question<String> strictStringQuestion = new StrictString(type,
                            question.getString("question"),
                            question.getString("answer"));
                    questionList.add(strictStringQuestion);
                    break;
                case MultipleChoice:
                    JsonArray rawChoices = question.getJsonArray("choices");
                    ArrayList<String> choices = new ArrayList<>();
                    for (int j = 0; j < rawChoices.size(); j++) {
                        choices.add(rawChoices.getString(j));
                    }

                    Question<Integer> multipleChoiceQuestion = new MultipleChoice(type,
                            question.getString("question"),
                            choices,
                            question.getInt("answer"));
                    questionList.add(multipleChoiceQuestion);
                    break;
                case TrueFalse:
                    Question<Boolean> trueFalseQuestion = new TrueFalse(type,
                            question.getString("question"),
                            question.getBoolean("answer"));
                    questionList.add(trueFalseQuestion);
                    break;
                case Invalid:
                    System.out.println("Invalid question:");
                    System.out.println(question);
                default:
                    System.out.println("UH OH! Hit the default!");
            }
        }
        return questionList;
    }
}
