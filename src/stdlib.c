#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

typedef char* string;

typedef struct {
    string* data;
    size_t size;
    size_t capacity;
} VecString;

void initVecString(VecString *vector) {
    vector->size = 0;
    vector->capacity = 10;
    vector->data = malloc(vector->capacity * sizeof(string));
    if (vector->data == NULL) {
        fprintf(stderr, "Failed to allocate memory\n");
        exit(EXIT_FAILURE);
    }
}

VecString newVecString() {
    VecString vector;
    initVecString(&vector);
    return vector;
}

void resizeVecString(VecString *vector) {
    vector->capacity *= 2;
    string* newData = realloc(vector->data, vector->capacity * sizeof(string));
    if (newData == NULL) {
        fprintf(stderr, "Failed to allocate memory\n");
        exit(EXIT_FAILURE);
    }
    vector->data = newData;
}

void pushVecString(VecString *vector, const string str) {
    if (vector->size >= vector->capacity) {
        resizeVecString(vector);
    }
    vector->data[vector->size] = strdup(str);
    if (vector->data[vector->size] == NULL) {
        fprintf(stderr, "Failed to allocate memory for string\n");
        exit(EXIT_FAILURE);
    }
    vector->size++;
}

void modVecString(const VecString *vector, size_t index, const string new_val) {
    vector->data[index] = new_val;
}

const string getVecString(const VecString *vector, size_t index) {
    if (index >= vector->size) {
        fprintf(stderr, "Index out of bounds\n");
        return NULL;
    }
    return vector->data[index];
}

void freeVecString(VecString *vector) {
    for (size_t i = 0; i < vector->size; i++) {
        free(vector->data[i]);
    }
    free(vector->data);
}

string joinVecString(VecString *vector, const char delim) {
    if (vector->size == 0) {
        string emptyStr = malloc(1);
        if (emptyStr == NULL) {
            fprintf(stderr, "Failed to allocate memory for empty string\n");
            exit(EXIT_FAILURE);
        }
        emptyStr[0] = '\0';
        return emptyStr;
    }

    size_t totalLen = 0;
    for (size_t i = 0; i < vector->size; i++) {
        totalLen += strlen(vector->data[i]);
    }

    totalLen += vector->size - 1 + 1;

    string result = malloc(totalLen);
    if (result == NULL) {
        fprintf(stderr, "Failed to allocate memory for joined string\n");
        exit(EXIT_FAILURE);
    }

    result[0] = '\0';
    for (size_t i = 0; i < vector->size; i++) {
        strcat(result, vector->data[i]);
        if (i < vector->size - 1) {
            size_t len = strlen(result);
            result[len] = delim;
            result[len + 1] = '\0';
        }
    }

    return result;
}

VecString stringSplit(const string str, char delimiter) {
    VecString vec;
    initVecString(&vec);

    size_t len = strlen(str);
    string token = malloc(len + 1);
    if (token == NULL) {
        fprintf(stderr, "Failed to allocate memory for token\n");
        exit(EXIT_FAILURE);
    }

    size_t tokenLen = 0;
    for (size_t i = 0; i <= len; i++) {
        if (str[i] == delimiter || str[i] == '\0') {
            token[tokenLen] = '\0';
            pushVecString(&vec, token);
            tokenLen = 0;
        } else {
            token[tokenLen++] = str[i];
        }
    }

    if (str[len - 1] == delimiter) {
        token[tokenLen] = '\0';
        pushVecString(&vec, token);
    } else {
        free(token);
    }

    return vec;
}

VecString stringSplitWhitespace(const char* str) {
    VecString vec;
    initVecString(&vec);

    size_t len = strlen(str);
    char* token = malloc(len + 1);
    if (token == NULL) {
        fprintf(stderr, "Failed to allocate memory for token\n");
        exit(EXIT_FAILURE);
    }

    size_t tokenLen = 0;
    for (size_t i = 0; i <= len; i++) {
        if (isspace(str[i]) || str[i] == '\0') {
            if (tokenLen > 0) {
                token[tokenLen] = '\0';
                pushVecString(&vec, token);
                tokenLen = 0;
            }
        } else {
            token[tokenLen++] = str[i];
        }
    }

    if (tokenLen > 0) {
        token[tokenLen] = '\0';
        pushVecString(&vec, token);
    }

    free(token);

    return vec;
}

string stringTrim(const string str) {
    if (str == NULL) {
        return NULL;
    }

    string start = str;
    while (isspace((unsigned char)*start)) {
        start++;
    }

    if (*start == '\0') {
        string emptyStr = malloc(1);
        if (emptyStr == NULL) {
            fprintf(stderr, "Failed to allocate memory\n");
            exit(EXIT_FAILURE);
        }
        *emptyStr = '\0';
        return emptyStr;
    }

    string end = start + strlen(start) - 1;
    while (end > start && isspace((unsigned char)*end)) {
        end--;
    }

    size_t len = end - start + 1;

    string trimmedStr = malloc(len + 1);
    if (trimmedStr == NULL) {
        fprintf(stderr, "Failed to allocate memory\n");
        exit(EXIT_FAILURE);
    }

    strncpy(trimmedStr, start, len);
    trimmedStr[len] = '\0';

    return trimmedStr;
}

bool stringEmpty(const string str) {
    return strcmp(stringTrim(str), "") == 0;
}

bool stringCmp(const string str1, const string str2) {
    if (strcmp(str1, str2) == 0) {
        return true;
    }
    return false;
}

ulong djb2Hash(const string s) {
    unsigned long hash = 5381;
    for (; *s; ++s)
        hash = ((hash << 5) + hash) ^ *s;
    return hash;
}

string readToString(const string filename) {
    FILE *file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file");
        exit(1);
    }

    fseek(file, 0, SEEK_END);
    long fileSize = ftell(file);
    if (fileSize == -1) {
        perror("Error determining file size");
        fclose(file);
        exit(1);
    }
    fseek(file, 0, SEEK_SET);

    string buffer = (string)malloc(fileSize + 1);
    if (buffer == NULL) {
        perror("Error allocating memory");
        fclose(file);
        exit(1);
    }

    size_t bytesRead = fread(buffer, 1, fileSize, file);
    if (bytesRead != fileSize) {
        perror("Error reading file");
        free(buffer);
        fclose(file);
        exit(1);
    }

    buffer[fileSize] = '\0';

    fclose(file);

    return buffer;
}

VecString readToLines(const string filename) {
    string fcontent = readToString(filename);
    return stringSplit(fcontent, '\n');
}

VecString readToLinesNonEmpty(const string filename) {
    string fcontent = readToString(filename);
    VecString lines = stringSplit(fcontent, '\n');
    VecString nonEmptyLines = newVecString();
    for (int i=0; i<lines.size; i++) {
        char* line = getVecString(&lines, i);
        if (!stringEmpty(line)) {
            pushVecString(&nonEmptyLines, line);
        }
    }
    return nonEmptyLines;
}

VecString splitToLines(const string str) {
    return stringSplit(str, '\n');
}

void helloworld() {
    printf("Hello, World!\n");
}
