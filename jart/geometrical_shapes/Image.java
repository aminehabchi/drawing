package geometrical_shapes;

import java.awt.image.BufferedImage;
import java.awt.Graphics;
import java.awt.Color;
import javax.imageio.ImageIO;

import intarfaces.Displayable;

import java.io.File;
import java.io.IOException;

public class Image implements Displayable {

    private int width;
    private int height;
    private BufferedImage image;
    private Graphics g;
    private Color defaultColor = Color.BLUE; 

    public Image(int width, int height) {
        this.width = width;
        this.height = height;
        this.image = new BufferedImage(width, height, BufferedImage.TYPE_INT_ARGB);
        this.g = image.getGraphics();
        this.g.setColor(defaultColor);
        // Fill background with white
        g.setColor(Color.BLACK);
        g.fillRect(0, 0, width, height);
        g.setColor(defaultColor); 
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    @Override
    public void display(int x, int y, Color color) {
        g.setColor(color);
        g.drawLine(x, y, x, y);
    }

   
    public void save(String fileName) {
        try {
            ImageIO.write(image, "png", new File(fileName));
            System.out.println("Image saved as " + fileName);
        } catch (IOException e) {
            System.out.println("Error saving image: " + e.getMessage());
        } finally {
            g.dispose(); // dispose graphics context when done
        }
    }

    public void setDefaultColor(Color color) {
        this.defaultColor = color;
        g.setColor(defaultColor);
    }

    public BufferedImage getBufferedImage() {
        return image;
    }
}
