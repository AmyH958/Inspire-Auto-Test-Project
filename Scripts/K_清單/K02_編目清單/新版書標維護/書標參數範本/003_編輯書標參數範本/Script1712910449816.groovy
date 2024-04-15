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

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/input_(15trunk)_member_id'), GlobalVariable.admin_name)

WebUI.setEncryptedText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/input_(15trunk)_member_pwd'), GlobalVariable.admin_passwd)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/input_(15trunk)_Submit'))

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/BookLabelsOptionReport.page?init=1')

WebUI.maximizeWindow()

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/input__field6'), GlobalVariable.Book_title)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/input__list_temp_buttonSubmit'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/a_20230118'), GlobalVariable.Book_title)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/td_'), '台中總館')

WebUI.click(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/edit_img'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/tr_'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/td__1'), '輸出格式:')

WebUI.selectOptionByValue(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/select_PDFHTMLRTFCSV'), '1', true)

WebUI.click(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/input__RadioGroup_0_verticalRadio'))

WebUI.delay(1)

WebUI.click(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/input__RadioGroup_0_horizontalradio'))

WebUI.click(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/tr_'))

WebUI.setText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/input__topMargin', [('') : top_Margin_input]), top_Margin_input)

WebUI.setText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/input__leftMargin'), left_Margin_input)

WebUI.setText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/input__columnWidth'), book_Label_Width)

WebUI.setText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/input__columnHeight'), book_Label_Height)

WebUI.setText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/input__columnSpacing'), bookLabel_Interval)

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/編輯書標參數範本_old/New/td__1_2_3_4_5'), '數量:')

WebUI.delay(2)

WebUI.selectOptionByIndex(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/select_12345678910'), '2', FailureHandling.STOP_ON_FAILURE)

WebUI.delay(2)

WebUI.scrollToElement(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/Modify_Save_Button'), 1)

WebUI.mouseOver(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/Modify_Save_Button'))

WebUI.click(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/New/Modify_Save_Button'))

WebUI.delay(1)

WebUI.verifyElementText(findTestObject('清單/編目清單/新版書標維護/編輯書標參數範本_old/p_'), '儲存成功')

WebUI.delay(1)

WebUI.takeFullPageScreenshotAsCheckpoint('修改成功')

WebUI.closeBrowser()

