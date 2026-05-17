import type {
  BookmarkSortKey,
  BookmarkSummary,
  ExtensionSortKey,
  ExtensionSummary,
  PasswordSiteSortKey,
  PasswordSiteSummary,
  AssociatedProfileSortKey,
  AssociatedProfileSummary,
  BookmarkAssociatedProfileSummary,
  ExtensionAssociatedProfileSummary,
  ProfileSortKey,
  ProfileSummary,
  SortDirection,
} from "../types/browser";

export function compareText(left: string, right: string) {
  return left.localeCompare(right, undefined, {
    sensitivity: "base",
    numeric: true,
  });
}

function directionMultiplier(direction: SortDirection) {
  return direction === "asc" ? 1 : -1;
}

function applyDirection(result: number, direction: SortDirection) {
  return result * directionMultiplier(direction);
}

function compareOptionalText(
  left: string | null | undefined,
  right: string | null | undefined,
  direction: SortDirection,
) {
  const leftValue = left?.trim() ?? "";
  const rightValue = right?.trim() ?? "";

  const leftEmpty = leftValue.length === 0;
  const rightEmpty = rightValue.length === 0;

  if (leftEmpty && rightEmpty) return 0;
  if (leftEmpty) return 1;
  if (rightEmpty) return -1;

  return applyDirection(compareText(leftValue, rightValue), direction);
}

export function compareProfileId(left: string, right: string) {
  const leftKey = profileSortKeyValue(left);
  const rightKey = profileSortKeyValue(right);
  if (leftKey.group !== rightKey.group) return leftKey.group - rightKey.group;
  if (leftKey.number !== rightKey.number) return leftKey.number - rightKey.number;
  return compareText(leftKey.text, rightKey.text);
}

export function sortProfiles(
  items: ProfileSummary[],
  sortKey: ProfileSortKey,
  direction: SortDirection,
) {
  const profiles = [...items];
  return profiles.sort((left, right) => {
    if (sortKey === "email") {
      return (
        compareOptionalText(left.email, right.email, direction) ||
        compareText(left.name, right.name) ||
        compareProfileId(left.id, right.id)
      );
    }
    if (sortKey === "id") {
      return applyDirection(compareProfileId(left.id, right.id), direction);
    }
    return applyDirection(compareText(left.name, right.name), direction) || compareProfileId(left.id, right.id);
  });
}

export function sortExtensions(
  items: ExtensionSummary[],
  sortKey: ExtensionSortKey,
  direction: SortDirection,
) {
  const extensions = [...items];
  return extensions.sort((left, right) => {
    if (sortKey === "id") {
      return applyDirection(compareText(left.id, right.id), direction);
    }
    return applyDirection(compareText(left.name, right.name), direction) || compareText(left.id, right.id);
  });
}

export function sortBookmarks(
  items: BookmarkSummary[],
  sortKey: BookmarkSortKey,
  direction: SortDirection,
) {
  const bookmarks = [...items];
  return bookmarks.sort((left, right) => {
    if (sortKey === "url") {
      return compareOptionalText(left.url, right.url, direction) || compareText(left.title, right.title);
    }
    return compareOptionalText(left.title, right.title, direction) || compareText(left.url, right.url);
  });
}

export function sortPasswordSites(
  items: PasswordSiteSummary[],
  sortKey: PasswordSiteSortKey,
  direction: SortDirection,
) {
  const passwordSites = [...items];
  return passwordSites.sort((left, right) => {
    if (sortKey === "url") {
      return compareOptionalText(left.url, right.url, direction) || compareText(left.domain, right.domain);
    }
    return compareOptionalText(left.domain, right.domain, direction) || compareText(left.url, right.url);
  });
}

export function sortAssociatedProfiles(
  items: (
    | AssociatedProfileSummary
    | BookmarkAssociatedProfileSummary
    | ExtensionAssociatedProfileSummary
  )[],
  sortKey: AssociatedProfileSortKey,
) {
  const profiles = [...items];
  return profiles.sort((left, right) => {
    if (sortKey === "name") {
      return compareText(left.name, right.name) || compareProfileId(left.id, right.id);
    }
    return compareProfileId(left.id, right.id) || compareText(left.name, right.name);
  });
}

function profileSortKeyValue(profileId: string) {
  if (profileId === "Default") {
    return { group: 0, number: 0, text: profileId };
  }

  const profileNumber = profileId.startsWith("Profile ")
    ? Number(profileId.slice("Profile ".length))
    : Number.NaN;

  if (!Number.isNaN(profileNumber)) {
    return { group: 1, number: profileNumber, text: profileId };
  }

  return { group: 2, number: Number.MAX_SAFE_INTEGER, text: profileId };
}
