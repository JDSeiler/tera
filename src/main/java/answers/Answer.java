package answers;

public class Answer<T> {
    private T value;

    public Answer(T value) {
        this.setValue(value);
    }

    public void setValue(T value) {
        this.value = value;
    }

    public T getValue() {
        return value;
    }
}
