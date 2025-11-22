// Register functionality
document.addEventListener("DOMContentLoaded", () => {
  const form = document.getElementById("register-form");

  form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const email = document.getElementById("email").value;
    const password = document.getElementById("password").value;
    const confirmPassword = document.getElementById("confirm-password").value;

    if (password !== confirmPassword) {
      showError("error-message", "Passwords do not match");
      return;
    }

    if (password.length < 6) {
      showError("error-message", "Password must be at least 6 characters long");
      return;
    }

    try {
      const response = await fetch("/api/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      const data = await response.json();

      if (response.ok) {
        showSuccess(
          "success-message",
          "Registration successful! Redirecting to login..."
        );
        setTimeout(() => {
          window.location.href = "/login";
        }, 2000);
      } else {
        showError("error-message", data.error || "Registration failed");
      }
    } catch (error) {
      showError("error-message", "Network error. Please try again.");
    }
  });
});
