import os
import sys
import json
import time
import random
from dotenv import load_dotenv
import undetected_chromedriver as uc
from selenium.webdriver import ChromeOptions, Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait

# load_dotenv()

TWITTER_ACCOUNT = os.getenv("TWITTER_ACCOUNT")
PASSWORD = os.getenv("TWITTER_PASSWORD")
X_EMAIL = os.getenv("X_EMAIL")
BASE_URL = "http://127.0.0.1:4000"
REGISTER_OR_LOGIN_ENDPOINT = f"{BASE_URL}/login"

if not all([TWITTER_ACCOUNT, PASSWORD, X_EMAIL]):
    raise ValueError("Missing environment variables: ensure TWITTER_ACCOUNT, TWITTER_PASSWORD, and X_EMAIL are set.")

def initialize_driver():
    options = ChromeOptions()
    options.add_argument("--start-maximized")
    options.add_argument('--disable-dev-shm-usage')
    options.add_argument('--no-sandbox')
    return uc.Chrome(headless=True, use_subprocess=False, browser_executable_path='/usr/bin/chromium', options=options)

def login_to_twitter(driver):
    driver.get("https://twitter.com/i/flow/login")
    wait = WebDriverWait(driver, 20)

    username_input = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="username"]')))
    username_input.send_keys(TWITTER_ACCOUNT)
    username_input.send_keys(Keys.ENTER)
    print("Sent Twitter account", file=sys.stderr)
    time.sleep(1)

    input_field = wait.until(
        EC.any_of(
            EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[name="password"]')),
            EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="on"]'))
        )
    )

    if input_field.get_attribute('autocomplete') == 'on':
        print("Entering email field", file=sys.stderr)
        input_field.send_keys(X_EMAIL)
        input_field.send_keys(Keys.ENTER)
        time.sleep(1)
        input_field = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="current-password"]')))

    print("Entering password", file=sys.stderr)
    input_field.send_keys(PASSWORD)
    input_field.send_keys(Keys.ENTER)
    time.sleep(5)

def save_auth_tokens(driver):
    ct0 = driver.get_cookie("ct0")["value"]
    auth_token = driver.get_cookie("auth_token")["value"]
    with open('cookies.env', 'w') as f:
        tokens = json.dumps({"ct0": ct0, "auth_token": auth_token}).replace('"', '\\"').replace(' ', '')
        f.write(f"X_AUTH_TOKENS={tokens}\n")
    print("Auth tokens saved", file=sys.stderr)

def handle_callback(driver):
    driver.get(REGISTER_OR_LOGIN_ENDPOINT)
    try:
        allow_button = WebDriverWait(driver, 10).until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[id="allow"]')))
        allow_button.click()
        print("Authorization granted", file=sys.stderr)
    except Exception as e:
        print(f"Error during callback authorization: {e}", file=sys.stderr)

if __name__ == "__main__":
    driver = initialize_driver()
    try:
        login_to_twitter(driver)
        save_auth_tokens(driver)
        handle_callback(driver)
    except Exception as e:
        print(f"An error occurred: {e}", file=sys.stderr)
    finally:
        driver.quit()