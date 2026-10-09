import { t } from '../composables/useI18n';

export const formatLastPlayed = (timestamp: number): string => {
  if (!timestamp) return t('instance.played_just_now');
  const now = Date.now();
  const diffMs = now - timestamp;
  if (diffMs < 0) return t('instance.played_just_now');
  const diffMinutes = Math.floor(diffMs / 60000);
  if (diffMinutes < 1) return t('instance.played_just_now');
  if (diffMinutes < 60) return t('instance.played_minutes_ago', { time: diffMinutes.toString() });
  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) return t('instance.played_hours_ago', { time: diffHours.toString() });
  const diffDays = Math.floor(diffHours / 24);
  if (diffDays < 30) return t('instance.played_days_ago', { time: diffDays.toString() });
  const diffMonths = Math.floor(diffDays / 30);
  if (diffMonths < 12) return t('instance.played_months_ago', { time: diffMonths.toString() });
  const diffYears = Math.floor(diffDays / 365);
  return t('instance.played_years_ago', { time: diffYears.toString() });
};
