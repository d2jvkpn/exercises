function updateDateTime() {
  const now = new Date();

  const dateStr = now.toLocaleDateString();
  const clockStr = now.toLocaleTimeString();

  const days = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
  const dayStr = days[now.getDay()];

  document.getElementById('today-date').textContent = `${dateStr}`;
  document.getElementById('today-clock').textContent = `${timeStr}`;
  document.getElementById('today-day').textContent = `${dayStr}`;
}

console.log("==> today.js");
updateDateTime();

setInterval(updateDateTime, 1000);
