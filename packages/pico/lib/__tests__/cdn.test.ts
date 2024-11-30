import assert from 'assert';
import debug from 'debug';

import { getThemeURI } from '../cdn';

const testLogger = debug('supra:pico:tests:cdn');

testLogger('Running CDN Tests');

/** Default */
assert(
  getThemeURI('default') ===
    'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css'
);

/** Default Classless */
assert(
  getThemeURI('default', true) ===
    'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.min.css'
);

/** Unpkg Default */
assert(
  getThemeURI('default', false, 'unpkg') ===
    'https://unpkg.com/@picocss/pico@2.0.6/css/pico.min.css'
);

/** Unpkg Default Classless */
assert(
  getThemeURI('default', true, 'unpkg') ===
    'https://unpkg.com/@picocss/pico@2.0.6/css/pico.classless.min.css'
);

testLogger('Finished CDN Tests!');
