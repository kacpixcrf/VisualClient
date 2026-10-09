import { QUICK_PLAY_MIN_MINOR } from '../constants/instance';

export const supportsQuickPlay = (version: string): boolean => {
  const parts = version.split('.');
  if (parts.length < 2) return false;
  const major = parseInt(parts[0], 10);
  const minor = parseInt(parts[1], 10);
  if (isNaN(major) || isNaN(minor)) return false;
  return major > 1 || (major === 1 && minor >= QUICK_PLAY_MIN_MINOR);
};

export const capitalizeLoader = (loader: string): string =>
  loader.charAt(0).toUpperCase() + loader.slice(1);
