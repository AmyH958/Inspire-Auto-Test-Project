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

WebUI.setText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a__1'), '已儲存的記錄')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a__1'))

WebUI.verifyElementClickable(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__SELECTALL'))

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__SELECTALL'))

WebUI.scrollToElement(findTestObject('查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a_email'), 2)

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/a_email'))

WebUI.switchToWindowUrl('http://10.11.20.15/inspireapp/myaccount/Bookmarks,l1234.sdirect?sp=0&sp=1&sp=2&sp=3&sp=4&sp=5&sp=6&sp=7&sp=8&sp=9&sp=10&sp=11&sp=12&sp=13&sp=14&sp=15&sp=16&sp=17&sp=18&sp=19&sp=20&sp=21&sp=22&sp=23&sp=24&sp=25&sp=26&sp=27&sp=28&sp=29&sp=30&sp=31&sp=32&sp=33&sp=34&sp=35&sp=36&sp=37&sp=38&sp=39')

WebUI.enableSmartWait()

WebUI.maximizeWindow()

WebUI.switchToWindowTitle('神資圖書館(15trunk機)')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__to'))

WebUI.setText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__to'), 'hclee@mitac.com.tw')

WebUI.setText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__subject'), '測試發送email')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/input__sent'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/div_(E-mail)'), '電子郵件信箱(發送E-mail通知)')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/div_'), '您的電子郵件己經寄出!')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/span_'), '關閉')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤發送email/Page_(15trunk)/span_'))

WebUI.switchToDefaultContent()

WebUI.enableSmartWait()

WebUI.closeBrowser()

