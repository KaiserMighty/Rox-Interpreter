# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -g

# Directories
SRC_DIR = compiler
BIN = rox

# Automatically find all .c files in the source directory
SRCS = $(wildcard $(SRC_DIR)/*.c)

# Convert source files to object files in the same directory
OBJS = $(SRCS:.c=.o)

# Default target
all: $(BIN)

# Link the executable and ensure it has execute permission
$(BIN): $(OBJS)
	$(CC) $(CFLAGS) $^ -o $@
	chmod +x $@

# Rule to build object files in the source directory
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up generated files
clean:
	rm -f $(SRC_DIR)/*.o $(BIN)

# Phony targets
.PHONY: all clean
