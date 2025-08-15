package geometrical_shapes;

import java.awt.Color;
import helpers.RandomNumber;
import helpers.Utils;
import intarfaces.Displayable;
import intarfaces.Drawable;

public class Circle implements Drawable {
    private Point center;
    private int radius;
    private Color color;

    public void setColor(Color color) {
        this.color = color;
    }

    public Point getCenter() {
        return this.center;
    }

    public int getReduis() {
        return this.radius;
    }

    public Circle(Point center, int radius) {
        this.center = center;
        this.radius = radius;
        this.color = Color.WHITE;
    }

    public Circle(Point center, int radius, Color color) {
        this.center = center;
        this.radius = radius;
        this.color = color;
    }

    public static Circle random(int w, int h) {
        Point center = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        int radius = RandomNumber.getRandomInt(Math.min(w, h) / 2);
        Color color = Utils.getRandomColor();
        return new Circle(center, radius, color);
    }

    @Override
    public void draw(Displayable displayable) {
        if (displayable instanceof Image) {
            Image img = (Image) displayable;
            int cx = center.getX();
            int cy = center.getY();

            for (int y = cy - radius; y <= cy + radius; y++) {
                for (int x = cx - radius; x <= cx + radius; x++) {
                    int dx = x - cx;
                    int dy = y - cy;
                    int distanceSq = dx * dx + dy * dy;
                    if (distanceSq >= radius * radius - radius && distanceSq <= radius * radius + radius) {
                        img.display(x, y, color);
                    }
                }
            }
        }
    }

    @Override
    public Color getColor() {
        return color;
    }
}
