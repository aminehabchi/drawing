package geometrical_shapes;

import java.awt.Color;

import helpers.RandomNumber;
import helpers.Utils;
import intarfaces.Displayable;
import intarfaces.Drawable;

public class Triangle implements Drawable {

    private Point p1;
    private Point p2;
    private Point p3;
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

    public Point getP3() {
        return this.p3;
    }

    public Triangle(Point p1, Point p2, Point p3) {
        this.p1 = p1;
        this.p2 = p2;
        this.p3 = p3;
        this.color = Color.WHITE;
    }

    public Triangle(Point p1, Point p2, Point p3, Color color) {
        this.p1 = p1;
        this.p2 = p2;
        this.p3 = p3;
        this.color = color;
    }

    public static Triangle random(int w, int h) {
        Point p1 = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        Point p2 = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        Point p3 = new Point(RandomNumber.getRandomInt(w), RandomNumber.getRandomInt(h));
        Color color = Utils.getRandomColor();
        return new Triangle(p1, p2, p3, color);
    }

    @Override
    public void draw(Displayable displayable) {
        Color color = getColor();

        Line l1 = new Line(p1, p2);
        Line l2 = new Line(p2, p3);
        Line l3 = new Line(p3, p1);

        l1.setColor(color);
        l2.setColor(color);
        l3.setColor(color);

        l1.draw(displayable);
        l2.draw(displayable);
        l3.draw(displayable);
    }

    @Override
    public Color getColor() {
        return color;
    }
}
