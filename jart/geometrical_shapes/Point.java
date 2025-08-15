package geometrical_shapes;

import java.awt.Color;

import helpers.RandomNumber;
import helpers.Utils;
import intarfaces.Displayable;
import intarfaces.Drawable;

public class Point implements Drawable {
    private int x;
    private int y;
    private Color color;

    public Point(int x, int y) {
        this.x = x;
        this.y = y;
        this.color = Color.WHITE;
    }

    public Point(int x, int y, Color color) {
        this.x = x;
        this.y = y;
        this.color = Color.WHITE;
    }

    public static Point random(int w, int h) {
        Color color = Utils.getRandomColor();
        return new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h), color);
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    @Override
    public void draw(Displayable displayable) {
        if (displayable instanceof Image) {
            Image img = (Image) displayable;
            img.display(x, y, getColor());
        }
    }

    @Override
    public Color getColor() {
        return color;
    }
}
