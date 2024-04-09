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

WebUI.navigateToUrl(GlobalVariable.Inspire_web)

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input_(15trunk)_member_id'), GlobalVariable.admin_name)

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input_(15trunk)_member_pwd'), GlobalVariable.admin_passwd)

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input_(15trunk)_Submit'))

WebUI.delay(2)

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/BookLabelsOptionReport.page?init=1')

WebUI.delay(2)

WebUI.maximizeWindow()

WebUI.scrollToElement(findTestObject('清單/編目清單/新版書標維護/新增書標範本/a_'), 2)

WebUI.mouseOver(findTestObject('清單/編目清單/新版書標維護/新增書標範本/a_'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/a_'))

WebUI.maximizeWindow()

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__labelModelName'))

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/td_'), '書標範本名稱:')

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__labelModelName'), GlobalVariable.Book_title)

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/td__1'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/td__1'), '自訂圖書館名稱:')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/tr_'))

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__libName'), GlobalVariable.Book_title)

WebUI.delay(2)

WebUI.scrollToElement(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/書標參數_修改儲存按鈕'), 2)

WebUI.delay(2)

WebUI.click(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/書標參數_修改儲存按鈕'))

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/p_'), '儲存成功')

WebUI.delay(2)

WebUI.takeFullPageScreenshotAsCheckpoint('新增成功')

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/a__1_2'), '取消')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/a__1_2'))

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__field6'), GlobalVariable.Book_title)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/td__1_2'), '書標範本名稱:')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__field6'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/input__list_temp_buttonSubmit'))

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('清單/編目清單/新版書標維護/新增書標範本/a_verify_book_title'), GlobalVariable.Book_title)

WebUI.verifyElementPresent(findTestObject('清單/編目清單/新版書標維護/新增書標範本/a_verify_book_title'), 1)

WebUI.verifyElementVisible(findTestObject('清單/編目清單/新版書標維護/新增書標範本/a_verify_book_title'))

WebUI.takeFullPageScreenshotAsCheckpoint('新增書標範本成功')

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/新增書標範本/a__1_2_3'))

WebUI.closeBrowser()

