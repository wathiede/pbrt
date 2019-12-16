# Copyright 2019 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

class Type:
    def __init__(self, wrapped_type, native_type, example_good=None, example_bad=None):
        self.wrapped_type = wrapped_type
        self.native_type = native_type
        self.example_good = example_good
        self.example_bad = example_bad

input_types = [
    Type('bool', 'bool', 'true', 'false'),
    Type('Float', 'Float', '1.', '2.'),
    Type('Int', 'isize', '1', '2'),
    Type('Point2f',
        'Point2f',
        'Point2f::from([1., 1.])',
        'Point2f::from([2., 2.])'),
    Type('Vector2f',
        'Vector2f',
        'Vector2f::from([1., 1.])',
        'Vector2f::from([2., 2.])'),
    Type('Point3f',
        'Point3f',
        'Point3f::from([1., 1., 1.])',
        'Point3f::from([2., 2., 2.])'),
    Type('Vector3f',
        'Vector3f',
        'Vector3f::from([1., 1., 1.])',
        'Vector3f::from([2., 2., 2.])'),
    Type('Normal3f',
        'Normal3f',
        'Normal3f::from([1., 1., 1.])',
        'Normal3f::from([2., 2., 2.])'),
    Type('Spectrum',
        'Spectrum',
        'Spectrum::from_rgb([1., 1., 1.])',
        'Spectrum::from_rgb([2., 2., 2.])'),
    Type('String',
        'String',
        '"found".to_string()',
        '"default".to_string()'), 
    Type('Texture',
        'String',
        '"found".to_string()',
        '"default".to_string()'), 
];

use_map = {
        'Normal3f': 'use pbrt::core::geometry::Normal3f;',
        'Point2f': 'use pbrt::core::geometry::Point2f;',
        'Point3f': 'use pbrt::core::geometry::Point3f;',
        'Vector2f': 'use pbrt::core::geometry::Vector2f;',
        'Vector3f': 'use pbrt::core::geometry::Vector3f;',
        'Spectrum': 'use pbrt::core::spectrum::Spectrum;',
}
