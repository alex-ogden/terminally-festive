import os
import time
import random

# Tree parameters
tree_height = 30
rows_before_break = tree_height // 3
stump_height = 5
stump_width = 5
tree_width = (tree_height * 2) - 1  # Calculate correct tree width based on height
required_snow_particles = 10
sleep_time_seconds = 0.125  # Sleep time in seconds (125ms)


def build_picture():
    picture = []

    # Build the tree
    current_width = 1
    for y in range(tree_height):
        if y % rows_before_break == 0:
            current_width -= 8
        # Create a row of empty spaces and fill in just the tree
        row = [' '] * tree_width
        space = (tree_width - current_width) // 2
        for x in range(space, space + current_width):
            row[x] = '*'
        picture.append(row)
        current_width += 2

    # Build the stump
    space = (tree_width - stump_width) // 2
    for y in range(stump_height):
        # Create a row of empty spaces and fill in just the stump
        row = [' '] * tree_width
        for x in range(space, space + stump_width):
            row[x] = '#'
        picture.append(row)

    # Display message
    message = "Merry Christmas!"
    space = (tree_width - len(message)) // 2
    for y in range(1, 4):
        row = [' '] * tree_width
        if y == 1 or y == 3:
            picture.append(row)
        else:
            char_counter = 0
            for x in range(space, space + len(message)):
                row[x] = message[char_counter]
                char_counter += 1

        picture.append(row)
    
    return picture


def draw_picture(picture):
    for row in picture:
        print(''.join(row))


def add_snow(picture):
    added_snow_particles = 0

    while added_snow_particles < required_snow_particles:
        rand_y = random.randint(0, tree_height + stump_height - 1)  # Random Y coordinate within the tree height
        rand_x = random.randint(0, tree_width - 1)  # Random X coordinate within the tree width

        # Add snow only to empty spaces
        if picture[rand_y][rand_x] == ' ':
            picture[rand_y][rand_x] = 'x'
            added_snow_particles += 1

# This doesn't have to be it's own function
# you can just add the code to the if __name__ == '__main__'
# section as that is the entry point of the program
def main():
    while True:
        # Build the tree and stump
        picture = build_picture()
        # Add snow at random, as long as it doesn't cover the tree/stump
        add_snow(picture)
        # Draw the tree/stump/snow to the screen
        draw_picture(picture)

        # Pause
        time.sleep(sleep_time_seconds)
        # Clear the console (use 'cls' for Windows)
        os.system('clear')

if __name__ == "__main__":
    main()
