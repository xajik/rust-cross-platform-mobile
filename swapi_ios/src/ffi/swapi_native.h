#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "swapi_core.h"

typedef struct {
  const char *name;
  const char *gender;
  const char *mass;
} PeopleNative;

typedef struct {
  PeopleNative *array;
  uintptr_t length;
} PeopleNativeWrapper;

typedef struct {
  void *owner;
  void (*onResult)(void *owner, const PeopleNativeWrapper *arg);
  void (*onError)(void *owner, const char *arg);
} PeopleCallback;

SwapiClient *create_swapi_client(void);

void free_swapi_client(SwapiClient *client);

void load_all_people(SwapiClient *client, PeopleCallback outer_listener);
