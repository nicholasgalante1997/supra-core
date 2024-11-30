import assert from 'assert';
import debug from 'debug';

import SupraPico from '../model';

const testLogger = debug('supra:pico:tests:model');

testLogger('Running Model Tests');

assert.ok(new SupraPico({}), 'Expected new SupraPico({}) to be truthy');

testLogger('Finished Model Tests!');
