#include <vector>
#include <iostream>
#include <random>
#include <thread>
#include <chrono>

#include <unistd.h>

// Tree parameters
int treeHeight = 15;
int stumpHeight = 3;
int stumpWidth = 5;
int treeWidth = (treeHeight * 2) + 1; // Calculate correct tree width based on height

// Function prototypes
void buildPicture(std::vector<std::vector<char> >& picture);
void drawPicture(const std::vector<std::vector<char> >& picture);
void addSnow(std::vector<std::vector<char> >& picture);

void buildPicture(std::vector<std::vector<char> >& picture)
{
    picture.clear(); // Clear any existing picture

    // Build the tree
    int currentWidth = 1;
    for (int y = 0; y < treeHeight; y++) {
        std::vector<char> row(treeWidth, ' ');
        int space = (treeWidth - currentWidth) / 2;
        for (int x = space; x < space + currentWidth; x++) {
            row[x] = '*';
        }
        picture.push_back(row);
        currentWidth += 2;
    }

    // Build the stump
    int space = (treeWidth - stumpWidth) / 2;
    for (int y = 0; y < stumpHeight; y++) {
        std::vector<char> row(treeWidth, ' ');
        for (int x = space; x < space + stumpWidth; x++) {
            row[x] = '*';
        }
        picture.push_back(row);
    }
}

void drawPicture(const std::vector<std::vector<char> >& picture)
{
    for (const auto& row : picture) {
        for (char ch : row) {
            std::cout << ch;
        }
        std::cout << std::endl;
    }
}


void addSnow(std::vector<std::vector<char> >& picture)
{
    int requiredSnowParticles = 5;
    int addedSnowParticles = 0;

    while (addedSnowParticles < requiredSnowParticles) {
        int randY = rand() % treeHeight; // Random Y coordinate within the tree height
        int randX = rand() % treeWidth;  // Random X coordinate within the tree width

        // Add snow only to empty spaces
        if (picture[randY][randX] == ' ') {
            picture[randY][randX] = 'x';
            addedSnowParticles++;
        }
    }
}

int main()
{
    std::vector<std::vector<char> > picture;

    while (true) {
        buildPicture(picture);
        addSnow(picture);
        drawPicture(picture);

        // Pause for half a second
				usleep(75000);
				system("clear");
    }

    return 0;
}
