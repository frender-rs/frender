// TODO: date.toISOString() might have leading `-` or `+` but input value doesn't allow that.

function numberToDateString(v) {
  const d = new Date(v);
  const s = d.toISOString();
  const date = s.slice(0, s.indexOf("T"));
  return date;
}

function numberToMonthString(v) {
  const d = new Date(0);
  d.setUTCMonth(v);
  const s = d.toISOString();
  const month = s.slice(0, s.indexOf("T") - 3);
  return month;
}

function thursdayOfTheWeek(date) {
  const d = new Date(date);
  const day = d.getUTCDay();
  const offset = day === 0 ? -3 : 4 - day;
  d.setUTCDate(d.getUTCDate() + offset);
  return d;
}

function numberToWeekString(v) {
  const d = thursdayOfTheWeek(v);

  const firstWeek = new Date(d);
  firstWeek.setUTCMonth(0, 1);
  firstWeek.setUTCHours(0, 0, 0, 0);
  const day = firstWeek.getUTCDay();
  if (day >= 1 && day <= 4) {
    // this is the first week
    firstWeek.setUTCDate(firstWeek.getUTCDate() - day + 4);
  } else {
    // next week is the first week
    firstWeek.setUTCDate(firstWeek.getUTCDate() - day + 4 + 7);
  }

  const weekNumber =
    Math.floor((d.getTime() - firstWeek.getTime()) / 604800000) + 1; // 604800000 is seven days = 7 * 24 * 60 * 60 * 1000

  const weekNumberString = (weekNumber < 10 ? "0" : "") + weekNumber.toFixed(0);
  const weekString =
    firstWeek.getUTCFullYear().toString(10) + "-W" + weekNumberString;

  return weekString;
}

function numberToTimeString(v) {
  const d = new Date(v);
  const s = d.toISOString();
  const date = s.slice(s.indexOf("T") + 1, s.indexOf("Z"));
  return date;
}

function numberToDtString(v) {
  const d = new Date(v);
  const s = d.toISOString();
  const date = s.slice(0, s.indexOf("Z"));
  return date;
}

function numberValidAsInputValue(inputType, value) {
  switch (inputType) {
    case "date":
      return numberToDateString(value);
    case "month":
      return numberToMonthString(value);
    case "week":
      return numberToWeekString(value);
    case "time":
      return numberToTimeString(value);
    case "datetime-local":
      return numberToDtString(value);
    case "number":
    case "range":
      return String(value);
    default:
      return "";
  }
}

export function numberAsInputValue(inputType, value) {
  return Number.isNaN(value) ? "" : numberValidAsInputValue(inputType, value);
}

export function setDefaultValueAsNumber(input, defaultValue) {
  input.defaultValue = Number.isNaN(defaultValue)
    ? ""
    : numberValidAsInputValue(input.type, defaultValue);
}
