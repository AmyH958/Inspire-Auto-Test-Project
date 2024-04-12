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
import java.lang.String as String

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.Inspire_web)

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/input_(15trunk)_member_id'), GlobalVariable.admin_name)

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/input_(15trunk)_member_pwd'), GlobalVariable.admin_passwd)

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/input_(15trunk)_Submit'))

WebUI.scrollToElement(findTestObject('清單/編目清單/新版書標維護/書標參數範本/a_'), 0)

WebUI.delay(2)

WebUI.waitForElementClickable(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/a_清單'), 5)

WebUI.click(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/a_清單'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_'), '清單')

WebUI.delay(2)

WebUI.maximizeWindow()

WebUI.scrollToPosition(75, 36)

WebUI.getPageHeight()

WebUI.getPageWidth()

WebUI.scrollToElement(findTestObject('清單/編目清單/新版書標維護/書標參數範本/a__1'), 5)

WebUI.waitForElementPresent(findTestObject('清單/編目清單/新版書標維護/書標參數範本/a__1'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.mouseOver(findTestObject('清單/編目清單/新版書標維護/書標參數範本/a__1'), FailureHandling.STOP_ON_FAILURE)

WebUI.enableSmartWait()

WebUI.getElementHeight(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/a_編目清單'))

WebUI.getElementLeftPosition(findTestObject('清單/編目清單/新版書標維護/Page_(15trunk)/a_編目清單'))

WebUI.delay(2)

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/BookLabelsOptionReport.page?init=1')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/b_'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/input__list_temp_buttonSubmit'))

WebUI.delay(2)

WebUI.takeFullPageScreenshotAsCheckpoint('Page 1 result')

WebUI.rightClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_2'))

WebUI.verifyElementPresent(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_2'), 0)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_2'))

WebUI.takeFullPageScreenshotAsCheckpoint('Page 2 result')

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_- (sec)_list_temp_jumpLastPage'))

WebUI.verifyElementClickable(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a_- (sec)_list_temp_jumpLastPage'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/書標參數範本/a__1_2_3'))

WebUI.closeBrowser()

