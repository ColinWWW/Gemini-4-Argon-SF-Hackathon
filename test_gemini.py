import os
from dotenv import load_dotenv
from google import genai

# Load environment variables from .env
load_dotenv()

# Initialize the Gemini client (automatically reads GEMINI_API_KEY from environment)
client = genai.Client()

def main():
    # Model options: "gemini-4-argon", "gemini-3.8-flash", "gemini-3.1-pro-preview"
    model_name = "gemini-4-argon"

    print(f"Calling model '{model_name}'...")
    response = client.models.generate_content(
        model=model_name,
        contents="Introduce yourself and explain what you are capable of in 2 sentences.",
    )

    print("\n--- Model Response ---")
    print(response.text)

if __name__ == "__main__":
    main()
