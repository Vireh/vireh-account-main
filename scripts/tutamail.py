import undetected_chromedriver as uc
from selenium import webdriver
from selenium.webdriver import ChromeOptions, Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time
import random
import os
from dotenv import load_dotenv

TUTAMAIL_PASSWORD = os.getenv("TUTAMAIL_PASSWORD")
if not TUTAMAIL_PASSWORD:
    raise ValueError("Environment variable TUTAMAIL_PASSWORD not found.")

TUTAMAIL_EMAIL = os.getenv("TUTAMAIL_EMAIL")
if not TUTAMAIL_EMAIL:
    raise ValueError("Environment variable TUTAMAIL_EMAIL not found.")

def login_and_change_password():
    options = ChromeOptions()
    options.add_argument("--start-maximized")
    options.add_argument('--disable-dev-shm-usage')
    options.add_argument('--no-sandbox')
    
    driver = uc.Chrome(headless=True, use_subprocess=False, browser_executable_path='/usr/bin/chromium', options=options)
    wait = WebDriverWait(driver, 30)

    try:
        driver.get("https://app.tuta.com/login?noAutoLogin=true")
        wait.until(EC.presence_of_element_located((By.ID, "username"))).send_keys(TUTAMAIL_EMAIL)
        time.sleep(2)
        
        password_input = wait.until(EC.presence_of_element_located((By.ID, "password")))
        password_input.send_keys(TUTAMAIL_PASSWORD)
        password_input.submit()
        time.sleep(5)

        driver.get("https://app.tuta.com/settings/login")
        time.sleep(2)
        wait.until(EC.presence_of_element_located((By.XPATH, "//button[contains(text(), 'password')]"))).click()

        old_password_input = wait.until(EC.presence_of_element_located((By.ID, "password")))
        old_password_input.send_keys(TUTAMAIL_PASSWORD)
        old_password_input.submit()
        time.sleep(5)

        new_password = ''.join(random.choices('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=', k=16))
        print("New Password:", new_password)

        wait.until(EC.presence_of_element_located((By.ID, "newPassword"))).send_keys(new_password)
        wait.until(EC.presence_of_element_located((By.ID, "confirmPassword"))).send_keys(new_password)
        password_input.submit()
        time.sleep(15)

    except Exception as e:
        print("An error occurred:", e)

    finally:
        driver.quit()

if __name__ == "__main__":
    login_and_change_password()
