//
//  Created by ktiays on 2024/10/20.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

void *rqr_builder_create_with_data(const char *data, size_t length, int ecl,
                                   float width, float height);

void rqr_builder_build_path(void *builder, void *context,
                            void (*move_to_point_fn)(void *, float, float),
                            void (*line_to_point_fn)(void *, float, float),
                            void (*arc_to_fn)(void *, float, float, float,
                                              float, float, int),
                            void (*close_path_fn)(void *));

#ifdef __cplusplus
}
#endif