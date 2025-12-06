# Build stage
FROM rustlang/rust:nightly-slim AS builder

# Set working directory
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy web files BEFORE copying source (needed for compile-time include_str!)
COPY web ./web
COPY webappTemplates ./webappTemplates

# Copy source code
COPY src ./src

# Debug: List files before build
RUN echo "=== CHECKING FILES BEFORE BUILD ===" && \
    find web -name "*.html" -type f 2>/dev/null || echo "No HTML files found" && \
    ls -la web/templates/ 2>/dev/null || echo "No templates directory" && \
    ls -la web/static/ 2>/dev/null || echo "No static directory"

# Build for release with optimizations
RUN cargo build --release

# Runtime stage
FROM debian:sid-slim

# Install runtime dependencies and tools for health checks
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user FIRST
RUN groupadd -r appgroup && useradd -r -g appgroup appuser

# Set proper working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/aspiring-investments-backend ./aspiring-investments-backend

# Copy web files with proper structure
COPY web ./web
COPY webappTemplates ./webappTemplates

# Debug: Verify files were copied correctly
RUN echo "=== VERIFYING COPIED FILES ===" && \
    ls -la web/templates/ && \
    ls -la web/static/css/ && \
    ls -la web/static/js/ && \
    echo "=== FILES CHECK COMPLETE ==="

# Set ownership of ALL files to appuser
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose application port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./aspiring-investments-backend"]
