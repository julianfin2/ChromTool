import braveIcon from "../assets/brave.png";
import chromiumIcon from "../assets/chromium.png";
import chromeIcon from "../assets/google-chrome.png";
import edgeIcon from "../assets/microsoft-edge.png";
import settingsIcon from "../assets/settings.png";
import vivaldiIcon from "../assets/vivaldi.png";
import yandexIcon from "../assets/yandex.png";
import type {
  AssociatedProfileSummary,
  BookmarkAssociatedProfileSummary,
  ProfileSummary,
} from "../types/browser";

export const browserIconOptions = [
  { key: "chrome", label: "Google Chrome", src: chromeIcon },
  { key: "edge", label: "Microsoft Edge", src: edgeIcon },
  { key: "brave", label: "Brave", src: braveIcon },
  { key: "vivaldi", label: "Vivaldi", src: vivaldiIcon },
  { key: "yandex", label: "Yandex Browser", src: yandexIcon },
  { key: "chromium", label: "Chromium", src: chromiumIcon },
] as const;

export function browserIconSrc(iconKey: string | null | undefined) {
  return browserIconOptions.find((option) => option.key === iconKey)?.src ?? null;
}

export const configurationIconSrc = settingsIcon;

const avatarModules = import.meta.glob("../assets/avatars/*/*.png", {
  eager: true,
  import: "default",
}) as Record<string, string>;

const avatarMap = Object.entries(avatarModules).reduce<Record<string, Record<string, string>>>(
  (accumulator, [path, src]) => {
    const match = path.match(/avatars\/([^/]+)\/([^/]+)\.png$/);
    if (!match) return accumulator;

    const [, family, key] = match;
    if (!accumulator[family]) {
      accumulator[family] = {};
    }
    accumulator[family][key] = src;
    return accumulator;
  },
  {},
);

type ProfileAvatarLike =
  | Pick<
      ProfileSummary,
      | "avatarDataUrl"
      | "avatarIcon"
      | "defaultAvatarFillColor"
      | "defaultAvatarStrokeColor"
    >
  | Pick<
      AssociatedProfileSummary,
      | "avatarDataUrl"
      | "avatarIcon"
      | "defaultAvatarFillColor"
      | "defaultAvatarStrokeColor"
    >
  | Pick<
      BookmarkAssociatedProfileSummary,
      | "avatarDataUrl"
      | "avatarIcon"
      | "defaultAvatarFillColor"
      | "defaultAvatarStrokeColor"
    >;

const chromeProfileSvg = `
<svg version="1.1" width="96" height="96" viewBox="0 0 96 96" xmlns="http://www.w3.org/2000/svg">
  <circle cx="48" cy="48" r="48" fill="#{bg_rgb}"/>
  <g fill="#{fg_rgb}">
    <circle cx="48" cy="32" r="12"/>
    <path d="M24,68 C20,50 40,48 48,48 C56,48 76,50 72,68 Z"/>
  </g>
</svg>
`.trim();

const chromeGeneratedAvatarCache = new Map<string, string>();

export function profileAvatarSrc(
  profile: ProfileAvatarLike,
  browserFamilyId: string | null | undefined,
) {
  if (profile.avatarDataUrl) {
    return profile.avatarDataUrl;
  }

  const avatarFamilyId = resolveAvatarFamilyId(browserFamilyId);
  const avatarKey = normalizeAvatarIcon(profile.avatarIcon);
  if (avatarKey) {
    const familyMap = avatarFamilyId ? avatarMap[avatarFamilyId] : undefined;
    if (familyMap?.[avatarKey]) {
      return familyMap[avatarKey];
    }
  }

  if (avatarFamilyId === "chrome") {
    return createChromeGeneratedAvatar(
      profile.defaultAvatarFillColor,
      profile.defaultAvatarStrokeColor,
    );
  }

  return null;
}

function normalizeAvatarIcon(value: string | null | undefined) {
  if (!value) return null;
  const trimmed = value.trim();
  if (!trimmed) return null;
  const withoutQuery = trimmed.split("?")[0];
  const lastSegment = withoutQuery.split("/").pop() ?? withoutQuery;
  return lastSegment.replace(/\.png$/i, "");
}

function resolveAvatarFamilyId(browserFamilyId: string | null | undefined) {
  if (browserFamilyId === "chromium") {
    return "chrome";
  }

  return browserFamilyId ?? null;
}

function createChromeGeneratedAvatar(
  backgroundArgb: number | null | undefined,
  foregroundArgb: number | null | undefined,
) {
  if (backgroundArgb == null || foregroundArgb == null) {
    return null;
  }

  const cacheKey = `${backgroundArgb}:${foregroundArgb}`;
  const cached = chromeGeneratedAvatarCache.get(cacheKey);
  if (cached) {
    return cached;
  }

  const backgroundRgb = argbToRgbHex(backgroundArgb);
  const foregroundRgb = argbToRgbHex(foregroundArgb);
  const svg = chromeProfileSvg
    .replace("{bg_rgb}", backgroundRgb)
    .replace("{fg_rgb}", foregroundRgb);
  const dataUrl = `data:image/svg+xml;charset=UTF-8,${encodeURIComponent(svg)}`;

  chromeGeneratedAvatarCache.set(cacheKey, dataUrl);
  return dataUrl;
}

function argbToRgbHex(argbValue: number) {
  const unsignedArgb = argbValue >>> 0;
  const rgb = unsignedArgb & 0x00ff_ffff;
  return rgb.toString(16).padStart(6, "0");
}
