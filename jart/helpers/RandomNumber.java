package helpers;

import java.util.Random;

public class RandomNumber {

    private static Random random = new Random();

    public static int getRandomInt(int bound) {
        if (bound <= 0) {
            throw new IllegalArgumentException("Bound must be > 0");
        }
        return random.nextInt(bound);
    }
}
