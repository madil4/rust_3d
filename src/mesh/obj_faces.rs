extern crate nalgebra as na;

pub fn get() -> na::OMatrix<usize, na::Const<576>, na::Const<3>> {
    na::matrix![  1, 2, 0 ;  2, 4, 0 ;  4, 1, 0 ;  1, 3, 2 ;  3, 6, 2 ;  6, 4, 2 ;  5, 3, 1 ;  4, 5, 1 ;  5, 7, 3 ;  7, 6, 3 ;  10, 5, 4 ;  6, 10, 4 ;  7, 5, 8 ;  5, 10, 8 ;  9, 6, 7 ;  9, 10, 6 ;  11, 7, 8 ;  10, 11, 8 ;  18, 9, 7 ;  11, 12, 7 ;  12, 18, 7 ;  10, 9, 18 ;  24, 11, 10 ;  18, 24, 10 ;  14, 12, 11 ;  15, 14, 11 ;  24, 15, 11 ;  14, 37, 12 ;  37, 18, 12 ;  23, 13, 17 ;  13, 19, 17 ;  25, 13, 21 ;  21, 13, 16 ;  13, 25, 19 ;  16, 13, 23 ;  19, 14, 15 ;  33, 37, 14 ;  14, 20, 33 ;  20, 14, 19 ;  24, 36, 15 ;  36, 41, 15 ;  41, 19, 15 ;  23, 17, 31 ;  19, 31, 17 ;  24, 18, 37 ;  37, 36, 24 ;  25, 21, 27 ;  42, 21, 29 ;  42, 44, 21 ;  21, 16, 29 ;  27, 21, 44 ;  22, 20, 25 ;  25, 30, 22 ;  22, 39, 33 ;  39, 22, 26 ;  22, 33, 20 ;  22, 30, 32 ;  22, 32, 26 ;  25, 20, 19 ;  25, 27, 30 ;  33, 52, 37 ;  37, 41, 36 ;  37, 52, 41 ;  48, 39, 34 ;  39, 26, 34 ;  45, 66, 34 ;  26, 45, 34 ;  54, 48, 34 ;  66, 54, 34 ;  41, 56, 31 ;  31, 56, 43 ;  41, 31, 19 ;  23, 31, 43 ;  33, 39, 63 ;  39, 48, 63 ;  46, 64, 45 ;  45, 64, 74 ;  66, 45, 83 ;  83, 45, 74 ;  32, 46, 45 ;  32, 45, 26 ;  56, 55, 42 ;  56, 42, 59 ;  55, 44, 42 ;  42, 29, 35 ;  59, 42, 35 ;  50, 47, 38 ;  47, 50, 62 ;  53, 38, 47 ;  57, 53, 47 ;  62, 57, 47 ;  50, 40, 70 ;  70, 75, 50 ;  40, 50, 38 ;  50, 75, 62 ;  55, 56, 41 ;  43, 56, 69 ;  69, 56, 59 ;  58, 49, 55 ;  55, 49, 44 ;  52, 55, 41 ;  52, 65, 55 ;  58, 55, 65 ;  73, 53, 86 ;  53, 73, 59 ;  72, 86, 53 ;  72, 53, 57 ;  38, 53, 28 ;  28, 53, 59 ;  65, 77, 64 ;  74, 64, 77 ;  64, 58, 65 ;  46, 58, 64 ;  77, 65, 63 ;  77, 63, 78 ;  33, 63, 65 ;  78, 63, 48 ;  72, 68, 88 ;  68, 72, 57 ;  68, 85, 88 ;  85, 68, 76 ;  76, 68, 62 ;  62, 68, 57 ;  60, 80, 70 ;  80, 75, 70 ;  40, 60, 70 ;  81, 54, 71 ;  81, 71, 84 ;  71, 82, 89 ;  82, 71, 66 ;  54, 66, 71 ;  84, 71, 89 ;  78, 90, 77 ;  90, 74, 77 ;  80, 69, 73 ;  80, 73, 86 ;  73, 69, 59 ;  86, 72, 88 ;  60, 69, 80 ;  80, 86, 75 ;  93, 81, 99 ;  93, 67, 81 ;  81, 92, 99 ;  84, 92, 81 ;  81, 67, 54 ;  82, 95, 89 ;  95, 82, 66 ;  88, 85, 87 ;  85, 76, 87 ;  86, 88, 87 ;  97, 96, 105 ;  91, 96, 97 ;  106, 101, 97 ;  97, 101, 91 ;  106, 97, 105 ;  102, 98, 93 ;  93, 98, 90 ;  93, 99, 102 ;  78, 93, 90 ;  93, 78, 67 ;  83, 98, 95 ;  95, 98, 102 ;  66, 83, 95 ;  100, 95, 102 ;  95, 100, 89 ;  96, 103, 105 ;  103, 96, 94 ;  96, 91, 84 ;  94, 96, 84 ;  92, 101, 104 ;  116, 104, 101 ;  91, 101, 92 ;  106, 108, 101 ;  108, 116, 101 ;  90, 98, 83 ;  107, 103, 100 ;  115, 103, 107 ;  100, 103, 94 ;  103, 115, 105 ;  107, 99, 104 ;  107, 104, 116 ;  99, 92, 104 ;  107, 100, 102 ;  99, 107, 102 ;  116, 124, 107 ;  107, 124, 115 ;  113, 111, 109 ;  111, 121, 109 ;  122, 113, 109 ;  122, 109, 121 ;  110, 111, 112 ;  110, 119, 111 ;  112, 117, 110 ;  123, 119, 110 ;  110, 117, 128 ;  123, 110, 128 ;  113, 112, 111 ;  111, 119, 121 ;  126, 112, 113 ;  113, 122, 126 ;  127, 117, 112 ;  126, 127, 112 ;  118, 130, 114 ;  118, 114, 106 ;  137, 120, 114 ;  120, 106, 114 ;  137, 114, 130 ;  127, 119, 123 ;  127, 126, 119 ;  119, 126, 121 ;  133, 117, 127 ;  136, 128, 117 ;  133, 136, 117 ;  122, 121, 126 ;  130, 118, 144 ;  146, 118, 131 ;  131, 118, 115 ;  146, 144, 118 ;  118, 106, 105 ;  118, 105, 115 ;  123, 133, 127 ;  125, 135, 132 ;  135, 125, 149 ;  140, 145, 125 ;  153, 125, 145 ;  129, 125, 132 ;  129, 140, 125 ;  153, 149, 125 ;  134, 123, 128 ;  133, 123, 134 ;  137, 139, 120 ;  120, 139, 116 ;  120, 108, 106 ;  108, 120, 116 ;  132, 137, 130 ;  130, 144, 140 ;  130, 129, 132 ;  130, 140, 129 ;  141, 134, 128 ;  136, 148, 128 ;  148, 141, 128 ;  134, 142, 133 ;  136, 133, 142 ;  135, 157, 150 ;  157, 135, 149 ;  132, 135, 150 ;  138, 146, 131 ;  138, 131, 115 ;  154, 142, 134 ;  151, 134, 141 ;  151, 154, 134 ;  142, 148, 136 ;  137, 155, 139 ;  156, 155, 137 ;  156, 137, 132 ;  139, 155, 138 ;  139, 138, 116 ;  148, 142, 154 ;  138, 165, 146 ;  155, 165, 138 ;  116, 138, 124 ;  124, 138, 115 ;  141, 159, 151 ;  148, 159, 141 ;  167, 145, 164 ;  167, 153, 145 ;  164, 145, 140 ;  146, 162, 144 ;  162, 163, 144 ;  163, 164, 144 ;  164, 140, 144 ;  161, 162, 146 ;  146, 165, 161 ;  160, 151, 159 ;  154, 151, 160 ;  166, 159, 148 ;  154, 166, 148 ;  155, 179, 165 ;  168, 179, 155 ;  156, 168, 155 ;  180, 160, 159 ;  166, 174, 159 ;  174, 178, 159 ;  178, 180, 159 ;  173, 162, 161 ;  172, 161, 165 ;  169, 161, 172 ;  169, 173, 161 ;  157, 175, 170 ;  149, 175, 157 ;  157, 170, 156 ;  150, 157, 156 ;  171, 154, 160 ;  171, 160, 180 ;  177, 163, 162 ;  173, 177, 162 ;  186, 168, 163 ;  168, 164, 163 ;  177, 186, 163 ;  171, 166, 154 ;  171, 174, 166 ;  165, 179, 187 ;  187, 172, 165 ;  175, 167, 164 ;  153, 167, 175 ;  179, 168, 186 ;  170, 168, 156 ;  168, 170, 164 ;  177, 199, 186 ;  177, 173, 199 ;  179, 203, 187 ;  186, 203, 179 ;  182, 174, 171 ;  176, 182, 171 ;  188, 171, 180 ;  191, 171, 188 ;  191, 176, 171 ;  184, 178, 174 ;  182, 194, 174 ;  201, 184, 174 ;  194, 201, 174 ;  170, 175, 164 ;  149, 153, 175 ;  195, 178, 184 ;  180, 178, 192 ;  183, 192, 178 ;  183, 178, 195 ;  189, 185, 181 ;  181, 185, 176 ;  202, 189, 181 ;  191, 202, 181 ;  191, 181, 176 ;  185, 200, 182 ;  185, 182, 176 ;  194, 182, 198 ;  200, 198, 182 ;  201, 195, 184 ;  194, 180, 201 ;  194, 188, 180 ;  192, 201, 180 ;  189, 196, 185 ;  196, 222, 185 ;  222, 200, 185 ;  189, 219, 196 ;  202, 219, 189 ;  198, 200, 191 ;  198, 191, 188 ;  200, 202, 191 ;  194, 198, 188 ;  225, 192, 183 ;  192, 225, 213 ;  209, 201, 192 ;  192, 220, 206 ;  206, 209, 192 ;  192, 213, 220 ;  207, 197, 205 ;  211, 205, 197 ;  214, 197, 207 ;  214, 228, 197 ;  228, 204, 197 ;  204, 211, 197 ;  232, 222, 196 ;  219, 232, 196 ;  200, 222, 202 ;  237, 205, 233 ;  233, 205, 211 ;  205, 210, 207 ;  215, 205, 237 ;  205, 215, 210 ;  201, 208, 195 ;  195, 208, 231 ;  201, 253, 208 ;  208, 245, 231 ;  253, 245, 208 ;  222, 238, 202 ;  238, 219, 202 ;  227, 247, 214 ;  247, 258, 214 ;  227, 214, 207 ;  214, 258, 228 ;  216, 230, 218 ;  230, 216, 220 ;  228, 235, 216 ;  216, 235, 223 ;  228, 216, 204 ;  204, 216, 218 ;  216, 212, 220 ;  216, 223, 212 ;  232, 252, 222 ;  252, 238, 222 ;  240, 232, 219 ;  238, 244, 219 ;  224, 240, 219 ;  244, 224, 219 ;  239, 221, 254 ;  221, 239, 210 ;  254, 221, 226 ;  226, 221, 210 ;  225, 187, 229 ;  225, 229, 242 ;  225, 183, 187 ;  242, 213, 225 ;  255, 230, 257 ;  255, 274, 230 ;  218, 230, 234 ;  274, 234, 230 ;  257, 230, 220 ;  240, 260, 232 ;  260, 252, 232 ;  256, 264, 233 ;  251, 256, 233 ;  237, 233, 264 ;  251, 233, 211 ;  259, 272, 235 ;  266, 235, 272 ;  259, 235, 228 ;  235, 266, 223 ;  261, 229, 203 ;  229, 261, 265 ;  203, 229, 187 ;  242, 229, 265 ;  244, 238, 260 ;  238, 252, 260 ;  263, 260, 240 ;  240, 248, 263 ;  240, 224, 248 ;  267, 244, 260 ;  244, 267, 248 ;  224, 244, 248 ;  268, 239, 254 ;  268, 250, 239 ;  210, 239, 227 ;  239, 250, 227 ;  256, 247, 264 ;  258, 247, 256 ;  247, 250, 264 ;  250, 247, 227 ;  258, 256, 270 ;  270, 256, 251 ;  279, 260, 263 ;  267, 260, 279 ;  254, 275, 268 ;  275, 254, 264 ;  254, 237, 264 ;  226, 237, 254 ;  281, 255, 276 ;  273, 276, 255 ;  274, 255, 281 ;  273, 255, 257 ;  203, 217, 261 ;  217, 231, 261 ;  261, 231, 245 ;  261, 245, 265 ;  263, 269, 277 ;  269, 263, 248 ;  279, 263, 277 ;  268, 275, 264 ;  250, 268, 264 ;  278, 269, 267 ;  267, 269, 248 ;  279, 278, 267 ;  272, 282, 276 ;  276, 266, 272 ;  282, 272, 280 ;  280, 272, 271 ;  271, 272, 259 ;  278, 277, 269 ;  277, 278, 279 ;  281, 276, 282 ;  266, 276, 273 ;  284, 282, 280 ;  283, 284, 280 ;  274, 283, 280 ;  274, 280, 271 ;  282, 285, 281 ;  287, 283, 281 ;  281, 283, 274 ;  285, 287, 281 ;  286, 282, 284 ;  285, 282, 286 ;  283, 289, 284 ;  288, 289, 283 ;  287, 288, 283 ;  294, 290, 289 ;  289, 290, 284 ;  291, 292, 289 ;  291, 289, 288 ;  292, 294, 289 ;  290, 293, 287 ;  290, 287, 285 ;  287, 293, 288 ;  290, 294, 293 ;  290, 286, 284 ;  285, 286, 290 ;  291, 296, 292 ;  293, 295, 291 ;  293, 291, 288 ;  295, 296, 291 ;  296, 294, 292 ;  294, 298, 293 ;  297, 295, 293 ;  298, 297, 293 ;  296, 298, 294 ;  300, 296, 295 ;  297, 302, 295 ;  302, 300, 295 ;  301, 298, 296 ;  300, 299, 296 ;  299, 301, 296 ;  297, 298, 302 ;  301, 302, 298 ;  300, 301, 299 ;  301, 300, 302 ;  84, 91, 92 ;  90, 83, 74 ;  33, 65, 52 ;  30, 49, 46 ;  30, 27, 49 ;  49, 58, 46 ;  27, 44, 49 ;  87, 79, 86 ;  76, 79, 87 ;  100, 94, 89 ;  132, 150, 156 ;  187, 183, 172 ;  173, 183, 195 ;  183, 169, 172 ;  173, 169, 183 ;  186, 217, 203 ;  195, 231, 199 ;  195, 199, 173 ;  231, 217, 199 ;  199, 217, 186 ;  201, 209, 253 ;  212, 206, 220 ;  206, 212, 209 ;  210, 227, 207 ;  259, 258, 271 ;  228, 258, 259 ;  234, 251, 218 ;  251, 234, 274 ;  271, 270, 274 ;  258, 270, 271 ;  249, 273, 257 ;  257, 220, 249 ;  265, 245, 253 ;  29, 16, 35 ;  16, 28, 35 ;  16, 23, 28 ;  54, 67, 48 ;  60, 43, 69 ;  40, 23, 43 ;  60, 40, 43 ;  28, 59, 35 ;  23, 38, 28 ;  23, 40, 38 ;  218, 211, 204 ;  226, 215, 237 ;  79, 75, 86 ;  94, 84, 89 ;  251, 211, 218 ;  215, 226, 210 ;  253, 266, 273 ;  223, 266, 253 ;  270, 251, 274 ;  30, 46, 32 ;  48, 67, 78 ;  75, 79, 62 ;  79, 76, 62 ;  265, 273, 249 ;  242, 265, 249 ;  220, 242, 249 ;  265, 253, 273 ;  242, 220, 213 ;  209, 223, 253 ;  212, 223, 209]
}
