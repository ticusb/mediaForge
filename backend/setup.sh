#!/bin/bash

# MediaForge Database Setup Script

set -e

echo "🚀 MediaForge Database Setup"
echo "=============================="

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL is not installed. Please install PostgreSQL first."
    exit 1
fi

# Configuration
DB_NAME="mediaForge"
DB_USER="postgres"
DB_PASSWORD="password"
DB_HOST="localhost"
DB_PORT="5432"

echo ""
echo "📊 Creating database: $DB_NAME"

# Check if database exists
if psql -U $DB_USER -h $DB_HOST -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
    echo "⚠️  Database '$DB_NAME' already exists."
    read -p "Do you want to drop and recreate it? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🗑️  Dropping existing database..."
        psql -U $DB_USER -h $DB_HOST -c "DROP DATABASE IF EXISTS $DB_NAME;"
    else
        echo "✓ Using existing database"
        exit 0
    fi
fi

# Create database
echo "📝 Creating new database..."
psql -U $DB_USER -h $DB_HOST -c "CREATE DATABASE $DB_NAME;"

echo "✓ Database created successfully!"
echo ""
echo "🔧 Creating .env file..."

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    cat > .env << EOF
# Database Configuration
DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME

# Redis Configuration (optional for MVP)
REDIS_URL=redis://localhost:6379

# JWT Secret (CHANGE THIS IN PRODUCTION!)
JWT_SECRET=$(openssl rand -base64 32)

# Server Configuration
HOST=127.0.0.1
PORT=8080

# Storage Configuration
STORAGE_MODE=local
LOCAL_STORAGE_PATH=./data/uploads

# Quota Configuration
FREE_TIER_IMAGE_DAILY=10
FREE_TIER_VIDEO_DAILY=3
FREE_TIER_CONCURRENT=1
PRO_TIER_VIDEO_DAILY=50
PRO_TIER_CONCURRENT=5

# Processing Configuration
MAX_IMAGE_SIZE_MB=10
MAX_VIDEO_SIZE_MB=100
MAX_VIDEO_DURATION_SECONDS=30
MODEL_PATH=./models/u2net.onnx
TEMP_DIR=./data/temp

# Logging
RUST_LOG=info,media_processor_server=debug
EOF
    echo "✓ .env file created"
else
    echo "⚠️  .env file already exists, skipping..."
fi

echo ""
echo "📁 Creating required directories..."
mkdir -p data/uploads
mkdir -p data/temp
mkdir -p models
echo "✓ Directories created"

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Review and update .env file with your settings"
echo "2. Run 'cargo build' to compile the project"
echo "3. Run 'cargo run' to start the server"
echo ""
echo "The migrations will run automatically on first startup."