package geometrical_shapes;

import java.awt.Color;

import helpers.RandomNumber;
import helpers.Utils;
import intarfaces.Displayable;
import intarfaces.Drawable;

public class Line implements Drawable {
    private Point p1;
    private Point p2;
    private Color color;

    public void setColor(Color color) {
        this.color = color;
    }

    public Point getP1() {
        return this.p1;
    }

    public Point getP2() {
        return this.p2;
    }

    public Line(Point p1, Point p2) {
        this.p1 = p1;
        this.p2 = p2;
        this.color = Color.WHITE;
    }

    public Line(Point p1, Point p2, Color color) {
        this.p1 = p1;
        this.p2 = p2;
        this.color = color;
    }

    public static Line random(int w, int h) {
        Point p1 = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        Point p2 = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        Color color = Utils.getRandomColor();
        return new Line(p1, p2, color);
    }

    @Override
    public void draw(Displayable displayable) {
        if (displayable instanceof Image) {
            Image img = (Image) displayable;

            int dx = p2.getX() - p1.getX();
            int dy = p2.getY() - p1.getY();
            int steps = Math.max(Math.abs(dx), Math.abs(dy));

            double Xinc = (double) dx / steps;
            double Yinc = (double) dy / steps;

            double x = p1.getX();
            double y = p1.getY();
            for (int i = 0; i <= steps; i++) {
                img.display((int) Math.round(x), (int) Math.round(y), getColor());
                x += Xinc;
                y += Yinc;
            }
        }
    }

    @Override
    public Color getColor() {
        return color;
    }
}
