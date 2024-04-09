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

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input_(15trunk)_member_id'), GlobalVariable.admin_name)

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input_(15trunk)_member_pwd'), GlobalVariable.admin_passwd)

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input_(15trunk)_Submit'))

WebUI.delay(1)

WebUI.maximizeWindow()

WebUI.delay(2)

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/BookLabelsOptionReport.page?init=1')

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input__field6'), GlobalVariable.Book_title)

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input__field6'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input__list_temp_buttonSubmit'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/a_20230121'), GlobalVariable.Book_title)

WebUI.verifyElementPresent(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/img'), 0, FailureHandling.STOP_ON_FAILURE)

WebUI.maximizeWindow(FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/img'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/td_'), 0, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div_'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div_'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div_'), '確定要刪除此範本嗎?')

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/a__1_2_3'), '是')

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input__field6'), '', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/input__list_temp_buttonSubmit'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div_0   (   0.4 sec) 102050100'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div__1'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/div__1'), '新增')

WebUI.takeFullPageScreenshotAsCheckpoint('刪除書標範本成功')

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/刪除書標參數範本/a__1_2_3_4'))

WebUI.closeBrowser()

