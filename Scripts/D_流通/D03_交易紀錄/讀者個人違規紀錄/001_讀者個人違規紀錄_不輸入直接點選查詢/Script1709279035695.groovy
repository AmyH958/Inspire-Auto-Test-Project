import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a_'), '流通')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a__1'), '交易紀錄')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a__1_2'), '讀者個人違規記錄')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a__1_2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/select_'), '2', true)

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__TextField'), '李')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__buttonSubmit'))

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__TextField'), '')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/select_'), '1', true)

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__TextField'), 'A')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__buttonSubmit'))

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__TextField'), '')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/select__1'), '1', 
    true)

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__buttonSubmit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/td_'), '序號')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/span_1'), '1')

WebUI.takeFullPageScreenshotAsCheckpoint('查詢相對應資料')

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a_L124060080'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/div_Penalties'), 'Penalties')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/span_'), '讀者罰款交易明細')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/span__1'), '逾期')

WebUI.takeFullPageScreenshot()

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/img_Penalties_Any_3_0'))

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/input__resetbutton'))

WebUI.click(findTestObject('Object Repository/流通/交易紀錄/讀者個人違規紀錄/不輸入直接點選查詢/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

