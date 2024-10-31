import os
import sys
import undetected_chromedriver as uc
import time
from selenium.webdriver import ChromeOptions, Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait
import random
from dotenv import load_dotenv

load_dotenv()
TWITTER_ACCOUNT = os.getenv("TWITTER_ACCOUNT")
PASSWORD = os.getenv("TWITTER_PASSWORD")
X_EMAIL = os.getenv("X_EMAIL")

if not all([TWITTER_ACCOUNT, PASSWORD, X_EMAIL]):
    raise ValueError("Environment variables TWITTER_ACCOUNT, TWITTER_PASSWORD, or X_EMAIL not found.")

def initialize_driver():
    options = ChromeOptions()
    options.add_argument("--start-maximized")
    options.add_argument('--disable-dev-shm-usage')
    options.add_argument('--no-sandbox')
    return uc.Chrome(headless=True, use_subprocess=False, browser_executable_path='/usr/bin/chromium', options=options)

def login(driver):
    driver.get("https://twitter.com/i/flow/login")
    wait = WebDriverWait(driver, 20)

    username_input = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="username"]')))
    username_input.send_keys(TWITTER_ACCOUNT)
    username_input.send_keys(Keys.ENTER)
    print("Username entered", file=sys.stderr)

    input_field = wait.until(
        EC.any_of(
            EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[name="password"]')),
            EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="on"]'))
        )
    )

    if input_field.get_attribute('autocomplete') == 'on':
        print("Entering email", file=sys.stderr)
        input_field.send_keys(X_EMAIL)
        input_field.send_keys(Keys.ENTER)
        input_field = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="current-password"]')))

    print("Entering password", file=sys.stderr)
    input_field.send_keys(PASSWORD)
    input_field.send_keys(Keys.ENTER)
    time.sleep(5)

def change_password(driver, current_password):
    new_password = ''.join(random.choices('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=', k=16))
    print("New password:", new_password)

    driver.get("https://x.com/settings/password")
    wait = WebDriverWait(driver, 20)
    time.sleep(2)

    current_password_input = wait.until(EC.visibility_of_element_located((By.NAME, "current_password")))
    current_password_input.send_keys(current_password)
    
    new_password_input = wait.until(EC.visibility_of_element_located((By.NAME, "new_password")))
    new_password_input.send_keys(new_password)

    confirm_password_input = wait.until(EC.visibility_of_element_located((By.NAME, "password_confirmation")))
    confirm_password_input.send_keys(new_password)

    submit_button = wait.until(EC.visibility_of_element_located((By.XPATH, '/html/body/div[1]/div/div/div[2]/main/div/div/div/section[2]/div[2]/div[3]/button')))
    submit_button.click()
    print("Password change submitted", file=sys.stderr)
    time.sleep(10)

if __name__ == "__main__":
    driver = initialize_driver()
    try:
        login(driver)
        change_password(driver, PASSWORD)
    except Exception as e:
        print("An error occurred:", e, file=sys.stderr)
    finally:
        driver.quit()
