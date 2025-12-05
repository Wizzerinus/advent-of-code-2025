package day5;

import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Task {

    static class Ingredient {
        long index;

        Ingredient(String s) {
            index = Long.parseLong(s);
        }
    }

    static class Range {
        long min;
        long max;

        Range(String s) {
            var items = s.split("-");
            min = Long.parseLong(items[0]);
            max = Long.parseLong(items[1]);
        }

        Range(long min, long max) {
            this.min = min;
            this.max = max;
        }

        boolean contains(Ingredient i) {
            return min <= i.index && max >= i.index;
        }

        long size() {
            return max - min + 1;
        }

        long min() {
            return min;
        }

        @Override
        public String toString() {
            return min + "->" + max;
        }
    }

    private static List<Range> optimize(List<Range> inputs) {
        List<Range> output = new ArrayList<>();
        inputs.sort(Comparator.comparingLong(Range::min).thenComparingLong(Range::size));
        Range ongoing = new Range(-1, -2);
        for (var it : inputs) {
            if (it.min <= ongoing.max + 1) {
                ongoing.max = Math.max(it.max, ongoing.max);
            } else {
                if (ongoing.size() > 0)
                    output.add(ongoing);
                ongoing = it;
            }
        }
        if (ongoing.size() > 0)
            output.add(ongoing);

        return output;
    }

    private static void run(List<String> lines) {
        List<Range> ranges = new ArrayList<>();
        List<Ingredient> ingredients = new ArrayList<>();
        for (var l : lines) {
            if (l.contains("-")) {
                ranges.add(new Range(l));
            } else if (!l.isEmpty()) {
                ingredients.add(new Ingredient(l));
            }
        }

        ranges = optimize(ranges);

        var day1 = 0;
        for (var i : ingredients) {
            for (var r : ranges) {
                if (r.contains(i)) {
                    day1++;
                    break;
                }
            }
        }

        long day2 = 0;
        for (var i : ranges) {
            day2 += i.size();
        }

        System.out.println("Day 1: " + day1);
        System.out.println("Day 2: " + day2);
    }

    public static void main(String[] args) {
        try (var u = new FileReader("./day5/input5.txt")) {
            var lines = u.readAllLines();
            run(lines);
        } catch (IOException e) {
            System.out.println("Unable to open file");
            e.printStackTrace();
            return;
        }
    }
}