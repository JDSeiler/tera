/*
Basic rewrite plan:
- Review functionality of the Java version
- Define YAML file format
- Define default file storage location

Minimal amount of stuff:
The CLI will be very simple, I think. At a minimum you need:
1. A command to start a quiz
2. A command to list available quizzes (for ease-of-use)
3. (Nice stuff like help commands that the clap crate has)

There are more possibilities/features, but those are the minimum.

Then, there needs to be code that can read the question sets and
report errors in the file.
There must also be code that can give the interactive quiz and
report the grade you got.

At the start, question sets will be "flat". I will not set out to 
allow for "including" other question sets yet, since that's kinda
complicated. I would also be interested in storing performance over
time for question sets, or other statistics.

The actual UX of taking the quiz could also use some work. But I don't 
want to go whole hog into using something like Cursive or Iced. This 
project, ideally, doesn't take too much time. I just do the rewrite so 
and then move on to something more interesting.

*/

fn main() {
    println!("Hello, world!");
}
