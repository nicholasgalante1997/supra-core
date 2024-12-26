import { describe, test, expect } from 'bun:test';
import { getThemeURI } from '../cdn';

describe('lib/cdn.ts', () => {
  test('getThemeURI', () => {
    expect(getThemeURI('default')).toBe(
      'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css'
    );
  });

  test('getThemeURI with classless', () => {
    expect(getThemeURI('default', true)).toBe(
      'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.min.css'
    );
  });

  test('getThemeURI with unpkg', () => {
    expect(getThemeURI('default', false, 'unpkg')).toBe(
      'https://unpkg.com/@picocss/pico@2.0.6/css/pico.min.css'
    );
  });

  test('getThemeURI with unpkg and classless', () => {
    expect(getThemeURI('default', true, 'unpkg')).toBe(
      'https://unpkg.com/@picocss/pico@2.0.6/css/pico.classless.min.css'
    );
  });

  test('getThemeURI with a pico theme', () => {
    expect(getThemeURI('cyan')).toBe(
      'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.cyan.min.css'
    );
  });

  test('getThemeURI with a pico theme and classless', () => {
    expect(getThemeURI('cyan', true)).toBe(
      'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.cyan.min.css'
    );
  });

});
