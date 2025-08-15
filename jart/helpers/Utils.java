package helpers;

import java.awt.Color;

public class Utils {
    public static Color getRandomColor() {
        int r = RandomNumber.getRandomInt(256);
        int g = RandomNumber.getRandomInt(256);
        int b = RandomNumber.getRandomInt(256);
        return new Color(r, g, b);
    }
}