# Tera
Tera is a command line utility for drilling flash cards.
Sets of flash cards are defined using simple JSON files
stored in `~/.tera/sets`. An example file can be found in
`/src/main/resources/example-schema.json`.

# Installation
Run `mvn clean package` in the root of the repository 
to build `tera`, or `mvn clean compile assembly:single`
to both compile and package dependencies manually.

Next create the directory `~/.tera/sets` and copy 
`example-schema.json` into it. You should now be able
to run (from inside the `target` directory produced by maven):
`java -jar name-of-compiled-jar.jar example-schema` to
test out Tera.

The intention is for Tera to only ever be a single jar file,
so the installation process should be as simple as copying it
to the desired install location and creating a shell alias
for executing it with `java -jar`.

# Schema
The top level object must contain the following two properties:
1. `metadata`
2. `questions`

## metadata
Currently this object is not used for anything, but there are
plans to use its contents for configuring how Tera presents
the set of questions.

## questions
This array contains all of the questions in the set. Each question
is an object and can have one of 4 types:
1. `fuzzy-string`
2. `strict-string`
3. `multiple-choice`
4. `true-false`

All questions share 3 common properties:
1. `type` :: Defines the type of question as specified above.
2. `question` :: The text of the question as a string.
3. `answer` :: The answer to the question, can be different types depending on the `type` of question.

### fuzzy-string / strict-string
Both of these question types are identical except for how the answer is checked.
In a fuzzy-string, the answer is checked using a fuzzy-matching algorithm. So 
answers that are close to the correct answer within a high tolerance (95% match, subject to change)
will be correct. For instance, if the correct answer is `fuzzy matching` and the user types in `fuzzy-matching`
that will still register as correct since it's very close.

`strict-string` in contrast will only match if the provided answer exactly matches the correct answer.

### multiple-choice
This type of question contains an extra property, which is an array called `choices`. This array is
where you specify all the possible answers for your multiple choice question. The answer is an integer
which is the 0-based index of the correct answer. So if the correct answer is the 2nd element in the array,
the answer is the integer `1`.

### true-false
True/false questions are very similar to `strict-string` questions except the answer must be a JSON boolean (true or false).
Tera accepts a variety of answers that are truthy/falsey to humans, such as: `y`, `yes`, `false`, `NO`, etc. The whole list
is found in `/src/main/java/util/AnswerFetcher` and is very easy to extend.


